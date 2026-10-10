use crate::catalog::{Subsystem, ACTION_CATALOG};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSample {
    pub prompt: String,
    pub subsystem: Subsystem,
    pub action_id: u32,
    pub action_name: String,
}

pub struct DatasetGenerator;

impl DatasetGenerator {
    pub fn generate_curated_dataset() -> Vec<DatasetSample> {
        let mut samples = Vec::new();

        let prefixes = &[
            "",
            "por favor ",
            "che ",
            "podrias ",
            "quiero ",
            "necesito ",
            "kev ",
            "dale ",
        ];

        let suffixes = &[
            "",
            " por favor",
            " ya",
            " ahora",
            " rapido",
            " en el proyecto",
        ];

        for action in ACTION_CATALOG {
            // Añadir forma canónica
            samples.push(DatasetSample {
                prompt: action.canonical_es.to_string(),
                subsystem: action.subsystem,
                action_id: action.action_id,
                action_name: action.name.to_string(),
            });

            // Combinatorias con prefijos y sufijos para variaciones
            for var in action.variations {
                for p in prefixes {
                    for s in suffixes {
                        let prompt = format!("{}{}{}", p, var, s).trim().to_string();
                        if !prompt.is_empty() {
                            samples.push(DatasetSample {
                                prompt,
                                subsystem: action.subsystem,
                                action_id: action.action_id,
                                action_name: action.name.to_string(),
                            });
                        }
                    }
                }
            }
        }

        samples
    }
}
