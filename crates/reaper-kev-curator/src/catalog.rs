use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subsystem {
    Transport,
    Track,
    Navigation,
    Marker,
    Item,
    Options,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReaperActionDef {
    pub subsystem: Subsystem,
    pub action_id: u32,
    pub name: &'static str,
    pub canonical_es: &'static str,
    pub variations: &'static [&'static str],
}

pub static ACTION_CATALOG: &[ReaperActionDef] = &[
    // --- TRANSPORTE ---
    ReaperActionDef {
        subsystem: Subsystem::Transport,
        action_id: 1007,
        name: "Transport: Play",
        canonical_es: "reproducir",
        variations: &[
            "play",
            "dale play",
            "iniciar reproduccion",
            "arranca el play",
            "pone play",
            "comenza a reproducir",
            "dale reproducir",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Transport,
        action_id: 1016,
        name: "Transport: Stop",
        canonical_es: "detener",
        variations: &[
            "stop",
            "para",
            "detene la reproduccion",
            "frena",
            "frena la reproduccion",
            "para la reproduccion",
            "detenete",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Transport,
        action_id: 40044,
        name: "Transport: Play/stop",
        canonical_es: "play pausa",
        variations: &[
            "play stop",
            "alternar play y stop",
            "pausar o continuar",
            "toggle play",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Transport,
        action_id: 1013,
        name: "Transport: Record",
        canonical_es: "grabar",
        variations: &[
            "graba",
            "comenza a grabar",
            "iniciar grabacion",
            "pone a grabar",
            "empeza a grabar",
            "mandale grabacion",
            "grabar ahora",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Transport,
        action_id: 1068,
        name: "Transport: Toggle repeat",
        canonical_es: "repetir bucle",
        variations: &[
            "activar repeticion",
            "alternar loop",
            "repetir",
            "prender loop",
            "activar ciclo",
            "toggle repeat",
            "bucle activado",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Transport,
        action_id: 40042,
        name: "Transport: Go to start of project",
        canonical_es: "ir al inicio",
        variations: &[
            "rebobinar al inicio",
            "ir al principio del proyecto",
            "al inicio",
            "volver al comienzo",
            "anda al principio",
            "posicion cero",
        ],
    },

    // --- PISTAS (TRACKS) ---
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 6,
        name: "Track: Toggle mute for selected tracks",
        canonical_es: "mutear pistas",
        variations: &[
            "mutea la pista",
            "silenciar canal",
            "mutea las pistas seleccionadas",
            "silencia las pistas",
            "apaga el audio de la pista",
            "poner en silencio",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 7,
        name: "Track: Toggle solo for selected tracks",
        canonical_es: "solear pistas",
        variations: &[
            "solea la pista",
            "pone en solo la pista",
            "deja solo este canal",
            "solea las pistas seleccionadas",
            "activa solo",
            "escuchar solo esta pista",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 9,
        name: "Track: Toggle record arm for selected tracks",
        canonical_es: "armar para grabar",
        variations: &[
            "arma la pista para grabar",
            "arma este canal",
            "armar pistas seleccionadas",
            "habilita grabacion en esta pista",
            "activa grabacion en la pista",
            "armar pista",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 40001,
        name: "Track: Insert new track",
        canonical_es: "insertar pista",
        variations: &[
            "crea una pista nueva",
            "agrega una pista",
            "inserta un nuevo canal",
            "nueva pista",
            "pone otra pista",
            "anadi una pista",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 40005,
        name: "Track: Remove tracks",
        canonical_es: "eliminar pista",
        variations: &[
            "borra la pista",
            "elimina el canal seleccionado",
            "saca esta pista",
            "borra las pistas seleccionadas",
            "quitar pista",
        ],
    },

    // --- NAVEGACIÓN Y MARCADORES ---
    ReaperActionDef {
        subsystem: Subsystem::Marker,
        action_id: 40157,
        name: "Markers: Insert marker at current position",
        canonical_es: "insertar marcador",
        variations: &[
            "pone un marcador aca",
            "crea un marcador",
            "marca este punto",
            "inserta marca en la posicion actual",
            "agrega un marcador",
            "marca de tiempo aca",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Marker,
        action_id: 40172,
        name: "Markers: Go to previous marker/project start",
        canonical_es: "marcador anterior",
        variations: &[
            "ir al marcador anterior",
            "marca previa",
            "salta al marcador de atras",
            "volver al marcador previo",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Marker,
        action_id: 40173,
        name: "Markers: Go to next marker/project end",
        canonical_es: "marcador siguiente",
        variations: &[
            "ir al marcador siguiente",
            "proxima marca",
            "salta al siguiente marcador",
            "avanza al proximo marcador",
        ],
    },

    // --- OPCIONES / METRÓNOMO ---
    ReaperActionDef {
        subsystem: Subsystem::Options,
        action_id: 40364,
        name: "Options: Toggle metronome",
        canonical_es: "alternar metronomo",
        variations: &[
            "prende el metronomo",
            "apaga el metronomo",
            "activa el click",
            "saca el click",
            "toggle metronomo",
            "pone el click",
            "metronomo activado",
        ],
    },
];

pub fn find_action_by_id(id: u32) -> Option<&'static ReaperActionDef> {
    ACTION_CATALOG.iter().find(|a| a.action_id == id)
}
