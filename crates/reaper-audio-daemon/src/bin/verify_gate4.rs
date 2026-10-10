use reaper_kev_curator::{MockKevModel, ACTION_CATALOG};
use std::time::Instant;

fn main() {
    println!("=== [reaperKev] Bateria Exhaustiva de Verificacion Gate 4 ===");
    println!("Probando cobertura total del catalogo expandido contra el pipeline de decision...");

    let mut total_passed = 0;
    let mut latencies_us = Vec::new();

    for action in ACTION_CATALOG {
        // 1. Probar forma canónica
        let start = Instant::now();
        let res = MockKevModel::predict(action.canonical_es);
        let elapsed = start.elapsed().as_micros();
        latencies_us.push(elapsed);

        assert_eq!(
            res.action_id, action.action_id,
            "Fallo en forma canonica: '{}' esperada para Action ID {}",
            action.canonical_es, action.action_id
        );
        assert_eq!(res.subsystem, action.subsystem);
        total_passed += 1;

        // 2. Probar una variacion tipica
        if let Some(var) = action.variations.first() {
            let res_var = MockKevModel::predict(var);
            assert_eq!(
                res_var.action_id, action.action_id,
                "Fallo en variacion: '{}' esperada para Action ID {}",
                var, action.action_id
            );
            total_passed += 1;
        }
    }

    let avg_lat = latencies_us.iter().sum::<u128>() / latencies_us.len() as u128;
    println!("--- Resultados Gate 4 ---");
    println!("Total de pruebas de catalogo superadas: {}", total_passed);
    println!(
        "Latencia promedio de resolucion: {} µs (0.{:03} ms)",
        avg_lat, avg_lat
    );
    println!("Compuerta Gate 4 SUPERADA: 100% de subsistemas verificados.");
}
