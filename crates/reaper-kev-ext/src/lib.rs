pub mod ipc;
#[cfg(test)]
mod tests;

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use reaper_low::PluginContext;
use reaper_macros::reaper_extension_plugin;
use reaper_medium::{CommandId, ProjectContext, ReaperSession, UndoScope};

use ipc::{CommandPayload, CommandResponse};

#[cfg(windows)]
use windows::{
    core::PCWSTR,
    Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{
        ReadFile, WriteFile, PIPE_ACCESS_DUPLEX,
    },
    Win32::System::Pipes::{
        ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, PIPE_READMODE_BYTE,
        PIPE_TYPE_BYTE, PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
    },
};

static RUNNING: AtomicBool = AtomicBool::new(true);

static COMMAND_RECEIVER: Mutex<Option<Receiver<CommandPayload>>> = Mutex::new(None);
static REAPER_SESSION: Mutex<Option<ReaperSession>> = Mutex::new(None);

extern "C" fn on_reaper_timer() {
    if let Ok(rx_guard) = COMMAND_RECEIVER.lock() {
        if let Some(ref rx) = *rx_guard {
            while let Ok(cmd) = rx.try_recv() {
                if let Ok(session_guard) = REAPER_SESSION.lock() {
                    if let Some(ref session) = *session_guard {
                        let reaper = session.reaper();
                        let desc = cmd.description.as_deref().unwrap_or("reaperKev voice command");

                        reaper.undo_begin_block_2(ProjectContext::CurrentProject);
                        let command_id = CommandId::new(cmd.action_id);
                        reaper.main_on_command_ex(command_id, 0, ProjectContext::CurrentProject);
                        reaper.undo_end_block_2(ProjectContext::CurrentProject, desc, UndoScope::All);

                        let msg = format!("[reaperKev] Ejecutado Action ID {} ({})\n", cmd.action_id, desc);
                        reaper.show_console_msg(msg);
                    }
                }
            }
        }
    }
}

#[reaper_extension_plugin]
fn plugin_main(context: PluginContext) -> Result<(), Box<dyn Error>> {
    let mut session = ReaperSession::load(context);
    session
        .reaper()
        .show_console_msg("=== [reaperKev] Extension v0.1.0 inicializada ===\n");

    let (tx, rx): (Sender<CommandPayload>, Receiver<CommandPayload>) = unbounded();

    {
        let mut rx_guard = COMMAND_RECEIVER.lock().unwrap();
        *rx_guard = Some(rx);
    }

    session.plugin_register_add_timer(on_reaper_timer)?;

    #[cfg(windows)]
    start_named_pipe_server(tx);

    session.reaper().show_console_msg("[reaperKev] Servidor IPC iniciado en \\\\.\\pipe\\reaperkev_ipc\n");

    {
        let mut session_guard = REAPER_SESSION.lock().unwrap();
        *session_guard = Some(session);
    }

    Ok(())
}

#[cfg(windows)]
fn start_named_pipe_server(sender: Sender<CommandPayload>) {
    thread::spawn(move || {
        let pipe_name = "\\\\.\\pipe\\reaperkev_ipc\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        while RUNNING.load(Ordering::SeqCst) {
            unsafe {
                let handle = CreateNamedPipeW(
                    PCWSTR(pipe_name.as_ptr()),
                    PIPE_ACCESS_DUPLEX,
                    PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                    PIPE_UNLIMITED_INSTANCES,
                    4096,
                    4096,
                    0,
                    None,
                );

                if handle == INVALID_HANDLE_VALUE {
                    thread::sleep(std::time::Duration::from_millis(500));
                    continue;
                }

                let connected = ConnectNamedPipe(handle, None).is_ok();
                if connected {
                    let mut buffer = [0u8; 4096];
                    let mut bytes_read = 0u32;

                    if ReadFile(
                        handle,
                        Some(&mut buffer),
                        Some(&mut bytes_read),
                        None,
                    )
                    .is_ok()
                        && bytes_read > 0
                    {
                        if let Ok(payload) = serde_json::from_slice::<CommandPayload>(&buffer[..bytes_read as usize]) {
                            let _ = sender.send(payload);

                            let resp = CommandResponse {
                                success: true,
                                message: "Comando encolado para ejecución en REAPER".to_string(),
                            };

                            if let Ok(resp_bytes) = serde_json::to_vec(&resp) {
                                let mut bytes_written = 0u32;
                                let _ = WriteFile(
                                    handle,
                                    Some(&resp_bytes),
                                    Some(&mut bytes_written),
                                    None,
                                );
                            }
                        }
                    }

                    let _ = DisconnectNamedPipe(handle);
                }

                let _ = CloseHandle(handle);
            }
        }
    });
}
