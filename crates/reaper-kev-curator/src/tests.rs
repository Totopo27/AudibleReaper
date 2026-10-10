use crate::catalog::{find_action_by_id, Subsystem};
use crate::evaluator::{Evaluator, MockKevModel};
use crate::generator::DatasetGenerator;

#[test]
fn test_catalog_integrity() {
    let play_action = find_action_by_id(1007).expect("Action 1007 must exist");
    assert_eq!(play_action.subsystem, Subsystem::Transport);
    assert_eq!(play_action.canonical_es, "reproducir");
}

#[test]
fn test_dataset_generation_volume() {
    let dataset = DatasetGenerator::generate_curated_dataset();
    assert!(
        dataset.len() >= 1000,
        "Dataset debe superar las 1000 muestras"
    );
}

#[test]
fn test_evaluator_gate_threshold() {
    let dataset = DatasetGenerator::generate_curated_dataset();
    let report = Evaluator::evaluate(&dataset);

    assert!(
        report.accuracy_pct >= 95.0,
        "La precision evaluada debe ser >= 95%, obtenido: {:.2}%",
        report.accuracy_pct
    );
}

#[test]
fn test_unknown_command_rejection() {
    let unknown = MockKevModel::predict("haceme una pizza con queso");
    assert_eq!(unknown.subsystem, Subsystem::Unknown);
    assert_eq!(unknown.action_id, 0);
}
