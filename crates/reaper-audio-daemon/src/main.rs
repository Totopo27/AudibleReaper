pub mod earcons;
pub mod ptt;
pub mod capture;
pub mod ipc_client;
#[cfg(test)]
mod tests;

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use capture::AudioCapture;
use crossbeam_channel::unbounded;
use earcons::{EarconPlayer, EarconType};
use ipc_client::{CommandPayload, IpcClient};
use ptt::{PttEvent, PttManager};

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== [reaperKev] Audio Daemon v0.1.0 ===");
    println!("Iniciando sistema de captura, PTT (Teclado/MIDI) y Earcons...");

    let running = Arc::new(AtomicBool::new(true));

    let earcons = Arc::new(EarconPlayer::new()?);
    let capture = Arc::new(AudioCapture::new()?);
    let (ptt_tx, ptt_rx) = unbounded();

    let ptt_manager = PttManager::new(ptt_tx)?;
    ptt_manager.start_listeners();

    println!("Daemon listo. Atajos activos:");
    println!(" - Teclado: Alt + V (Push-to-Talk)");
    println!(" - MIDI: Pedal Sustain (CC64)");
    println!("Esperando interacciones...");

    while running.load(Ordering::SeqCst) {
        if let Ok(event) = ptt_rx.recv_timeout(Duration::from_millis(50)) {
            match event {
                PttEvent::Pressed => {
                    println!("[PTT] Activado -> Grabando audio...");
                    earcons.play(EarconType::PttStart);
                    capture.start();
                }
                PttEvent::Released => {
                    println!("[PTT] Liberado -> Procesando captura...");
                    earcons.play(EarconType::PttStop);
                    let audio_data = capture.stop();
                    println!("[Audio] Capturadas {} muestras PCM", audio_data.len());

                    // Tracer Bullet: si hay audio capturado, enviamos comando de prueba al pipe
                    if !audio_data.is_empty() {
                        let payload = CommandPayload {
                            action_id: 40044, // Play/Stop
                            track_index: None,
                            description: Some("Voice PTT Action: Play/Stop".to_string()),
                        };

                        match IpcClient::send_command(&payload) {
                            Ok(resp) => {
                                println!("[IPC] Respuesta exitosa de REAPER: {}", resp.message);
                                earcons.play(EarconType::Success);
                            }
                            Err(e) => {
                                eprintln!("[IPC] REAPER no responde en el pipe: {}", e);
                                earcons.play(EarconType::Error);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
