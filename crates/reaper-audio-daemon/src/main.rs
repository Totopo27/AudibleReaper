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
use reaper_kev_curator::{find_action_by_id, MockKevModel, Subsystem};

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== [reaperKev] Audio Daemon v0.1.0 ===");
    println!("Iniciando sistema de captura, PTT (Teclado/MIDI), Inferencia Kev y Earcons...");

    let running = Arc::new(AtomicBool::new(true));

    let earcons = Arc::new(EarconPlayer::new()?);
    let capture = Arc::new(AudioCapture::new()?);
    let (ptt_tx, ptt_rx) = unbounded();

    let ptt_manager = PttManager::new(ptt_tx)?;
    ptt_manager.start_listeners();

    println!("Daemon listo. Atajos activos:");
    println!(" - Teclado: Alt + V (Push-to-Talk)");
    println!(" - MIDI: Pedal Sustain (CC64)");
    println!("Esperando comandos de voz...");

    while running.load(Ordering::SeqCst) {
        if let Ok(event) = ptt_rx.recv_timeout(Duration::from_millis(50)) {
            match event {
                PttEvent::Pressed => {
                    println!("[PTT] Activado -> Abriendo microfono...");
                    earcons.play(EarconType::PttStart);
                    capture.start();
                }
                PttEvent::Released => {
                    println!("[PTT] Liberado -> Procesando audio...");
                    earcons.play(EarconType::PttStop);
                    let audio_data = capture.stop();
                    println!("[Audio] Capturadas {} muestras PCM", audio_data.len());

                    // Simulación / Transcripción (Tracer Bullet):
                    // En ausencia de pipeline Whisper pesado en local, evaluamos la frase patrón de prueba
                    let transcribed_text = "arma la pista para grabar";
                    println!("[STT] Transcripcion de voz: '{}'", transcribed_text);

                    // Pasar por el modelo Kev
                    let inference = MockKevModel::predict(transcribed_text);
                    println!(
                        "[Kev Engine] Inferencia resuelta -> Subsistema: {:?}, Action ID: {}, Confianza: {}%",
                        inference.subsystem, inference.action_id, inference.confidence
                    );

                    if inference.subsystem != Subsystem::Unknown && inference.action_id > 0 {
                        let action_desc = find_action_by_id(inference.action_id)
                            .map(|a| a.name)
                            .unwrap_or("Comando reaperKev");

                        let payload = CommandPayload {
                            action_id: inference.action_id,
                            track_index: None,
                            description: Some(format!("Kev Voice: {}", action_desc)),
                        };

                        match IpcClient::send_command(&payload) {
                            Ok(resp) => {
                                println!("[IPC] Ejecutado exitosamente en REAPER: {}", resp.message);
                                earcons.play(EarconType::Success);
                            }
                            Err(e) => {
                                eprintln!("[IPC] REAPER no disponible en el pipe: {}", e);
                                earcons.play(EarconType::Error);
                            }
                        }
                    } else {
                        println!("[Kev Engine] Comando fuera de dominio / descartado de forma segura.");
                        earcons.play(EarconType::Error);
                    }
                }
            }
        }
    }

    Ok(())
}
