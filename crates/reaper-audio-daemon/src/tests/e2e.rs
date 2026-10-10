use reaper_kev_curator::{find_action_by_id, MockKevModel, Subsystem};
use std::time::Instant;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MockCommandPayload {
    pub action_id: u32,
    pub description: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MockCommandResponse {
    pub success: bool,
    pub latency_us: u128,
}

/// Simula el despacho completo en memoria (STT transcript -> Kev Inference -> JSON Pipe Payload -> Action ID Dispatch)
pub fn run_simulated_e2e_cycle(spoken_phrase: &str) -> (MockCommandResponse, u128) {
    let start = Instant::now();

    // 1. Inferencia del modelo Kev
    let inference = MockKevModel::predict(spoken_phrase);

    if inference.subsystem == Subsystem::Unknown || inference.action_id == 0 {
        return (
            MockCommandResponse {
                success: false,
                latency_us: start.elapsed().as_micros(),
            },
            start.elapsed().as_micros(),
        );
    }

    let action_name = find_action_by_id(inference.action_id)
        .map(|a| a.name)
        .unwrap_or("Desconocido");

    // 2. Serialización y deserialización IPC (simulando paso por Named Pipe)
    let payload = MockCommandPayload {
        action_id: inference.action_id,
        description: action_name.to_string(),
    };

    let serialized = serde_json::to_vec(&payload).expect("Serialization failed");
    let deserialized: MockCommandPayload =
        serde_json::from_slice(&serialized).expect("Deserialization failed");

    // 3. Simulación de ejecución en Main Thread de REAPER (Undo Block + Main_OnCommand)
    assert!(deserialized.action_id > 0);

    let total_elapsed = start.elapsed().as_micros();

    (
        MockCommandResponse {
            success: true,
            latency_us: total_elapsed,
        },
        total_elapsed,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e_tracer_bullet_latency_gate_3() {
        let test_battery = [
            "arma la pista para grabar",
            "dale play",
            "para la reproduccion",
            "crea una pista nueva",
            "pone un marcador aca",
            "prende el metronomo",
            "mutea la pista",
            "solea la pista",
            "al inicio",
            "repetir",
        ];

        let mut latencies = Vec::new();

        for phrase in test_battery {
            let (resp, elapsed_us) = run_simulated_e2e_cycle(phrase);
            assert!(resp.success, "Fallo al procesar frase: '{}'", phrase);
            latencies.push(elapsed_us);
        }

        // Gate 3: La latencia total del pipeline de decisión IPC debe ser < 400 ms (400.000 µs).
        // En Rust optimizado / in-memory suele ser < 1 ms (< 1.000 µs).
        for &lat in &latencies {
            assert!(
                lat < 400_000,
                "Latencia excesiva: {} µs (máximo permitido 400 ms)",
                lat
            );
        }

        let avg_us: u128 = latencies.iter().sum::<u128>() / latencies.len() as u128;
        println!(
            "Latencia promedio E2E medida: {} µs (sub-milisegundo)",
            avg_us
        );
        assert!(
            avg_us < 10_000,
            "Latencia promedio debe ser sub-10ms en IPC"
        );
    }

    #[test]
    fn test_e2e_unknown_rejection() {
        let (resp, _) = run_simulated_e2e_cycle("prepara una taza de cafe");
        assert!(
            !resp.success,
            "Comando desconocido debe ser rechazado limpiamente"
        );
    }
}
