pub mod catalog;
pub mod evaluator;
pub mod generator;
#[cfg(test)]
mod tests;

use evaluator::Evaluator;
use generator::DatasetGenerator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== [reaperKev] Kev Model Curator & Dataset Generator ===");

    let dataset = DatasetGenerator::generate_curated_dataset();
    println!(
        "Generadas {} muestras curadas para la matriz cerrada.",
        dataset.len()
    );

    println!("Ejecutando evaluacion sobre el catalogo de transporte, pistas y opciones...");
    let report = Evaluator::evaluate(&dataset);

    println!("--- Reporte de Evaluacion ---");
    println!("Muestras totales: {}", report.total_samples);
    println!("Aciertos: {}", report.correct_predictions);
    println!("Precision (Accuracy): {:.2}%", report.accuracy_pct);

    if report.accuracy_pct >= 95.0 {
        println!("Compuerta Gate 2 SUPERADA: Precision >= 95%.");
    } else {
        println!("ADVERTENCIA: Precision por debajo del umbral del 95%.");
    }

    Ok(())
}
