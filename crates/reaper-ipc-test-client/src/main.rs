use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandPayload {
    pub action_id: u32,
    #[serde(default)]
    pub track_index: Option<u32>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub success: bool,
    pub message: String,
}

#[cfg(windows)]
use windows::{
    core::PCWSTR,
    Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{
        CreateFileW, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, OPEN_EXISTING,
    },
};

fn main() {
    println!("=== reaperKev IPC Test Client ===");

    #[cfg(windows)]
    {
        let pipe_name = "\\\\.\\pipe\\reaperkev_ipc\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let payload = CommandPayload {
            action_id: 40044, // 40044 = Transport: Play/stop
            track_index: None,
            description: Some("Test Play/Stop Tracer Bullet".to_string()),
        };

        let serialized = serde_json::to_vec(&payload).expect("Error serializando payload");

        unsafe {
            let handle = CreateFileW(
                PCWSTR(pipe_name.as_ptr()),
                GENERIC_READ.0 | GENERIC_WRITE.0,
                windows::Win32::Storage::FileSystem::FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            );

            if let Ok(handle) = handle {
                if handle != INVALID_HANDLE_VALUE {
                    let mut bytes_written = 0u32;
                    let write_res =
                        WriteFile(handle, Some(&serialized), Some(&mut bytes_written), None);

                    if write_res.is_ok() {
                        println!("Enviado payload de prueba: {:?}", payload);

                        let mut buf = [0u8; 4096];
                        let mut bytes_read = 0u32;
                        if ReadFile(handle, Some(&mut buf), Some(&mut bytes_read), None).is_ok() {
                            if let Ok(resp) = serde_json::from_slice::<CommandResponse>(
                                &buf[..bytes_read as usize],
                            ) {
                                println!("Respuesta del servidor IPC: {:?}", resp);
                            }
                        }
                    }

                    let _ = CloseHandle(handle);
                    return;
                }
            }

            println!("No se pudo conectar a \\\\.\\pipe\\reaperkev_ipc (¿Está REAPER abierto con reaper_kev.dll cargado?)");
        }
    }
}
