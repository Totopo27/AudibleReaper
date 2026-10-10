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

pub struct IpcClient;

impl IpcClient {
    #[cfg(windows)]
    pub fn send_command(payload: &CommandPayload) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        use windows::core::PCWSTR;
        use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
        use windows::Win32::Storage::FileSystem::{
            CreateFileW, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, OPEN_EXISTING,
        };

        let pipe_name = "\\\\.\\pipe\\reaperkev_ipc\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let serialized = serde_json::to_vec(payload)?;

        unsafe {
            let handle = CreateFileW(
                PCWSTR(pipe_name.as_ptr()),
                GENERIC_READ.0 | GENERIC_WRITE.0,
                windows::Win32::Storage::FileSystem::FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )?;

            if handle == INVALID_HANDLE_VALUE {
                return Err("No se pudo conectar al Named Pipe de REAPER".into());
            }

            let mut bytes_written = 0u32;
            WriteFile(handle, Some(&serialized), Some(&mut bytes_written), None)?;

            let mut buf = [0u8; 4096];
            let mut bytes_read = 0u32;
            ReadFile(handle, Some(&mut buf), Some(&mut bytes_read), None)?;

            let _ = CloseHandle(handle);

            let resp: CommandResponse = serde_json::from_slice(&buf[..bytes_read as usize])?;
            Ok(resp)
        }
    }

    #[cfg(not(windows))]
    pub fn send_command(_payload: &CommandPayload) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        Err("Named Pipes solo soportado en Windows".into())
    }
}
