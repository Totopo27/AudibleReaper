use crate::catalog::{ACTION_CATALOG, Subsystem};
use crate::generator::DatasetSample;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceResult {
    pub subsystem: Subsystem,
    pub action_id: u32,
    pub confidence: u32,
}

pub struct MockKevModel;

impl MockKevModel {
    pub fn predict(prompt: &str) -> InferenceResult {
        let normalized = prompt.to_lowercase().trim().to_string();

        let mut best_match: Option<(Subsystem, u32, usize)> = None;

        for action in ACTION_CATALOG {
            let canon = action.canonical_es.to_lowercase();
            if normalized.contains(&canon) {
                let len = canon.len();
                if best_match.as_ref().map_or(true, |(_, _, best_len)| len > *best_len) {
                    best_match = Some((action.subsystem, action.action_id, len));
                }
            }

            for var in action.variations {
                let v = var.to_lowercase();
                if normalized.contains(&v) {
                    let len = v.len();
                    if best_match.as_ref().map_or(true, |(_, _, best_len)| len > *best_len) {
                        best_match = Some((action.subsystem, action.action_id, len));
                    }
                }
            }
        }

        if let Some((subsystem, action_id, _)) = best_match {
            InferenceResult {
                subsystem,
                action_id,
                confidence: 98,
            }
        } else {
            InferenceResult {
                subsystem: Subsystem::Unknown,
                action_id: 0,
                confidence: 0,
            }
        }
    }
}

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(dataset: &[DatasetSample]) -> EvaluationReport {
        let mut total = 0;
        let mut correct = 0;

        for sample in dataset {
            total += 1;
            let result = MockKevModel::predict(&sample.prompt);
            if result.action_id == sample.action_id && result.subsystem == sample.subsystem {
                correct += 1;
            }
        }

        let accuracy = if total > 0 {
            (correct as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        EvaluationReport {
            total_samples: total,
            correct_predictions: correct,
            accuracy_pct: accuracy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvaluationReport {
    pub total_samples: usize,
    pub correct_predictions: usize,
    pub accuracy_pct: f64,
}
