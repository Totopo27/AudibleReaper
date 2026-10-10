use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subsystem {
    Transport,
    Track,
    Navigation,
    Marker,
    Item,
    Take,
    Envelope,
    TimeSelection,
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
    // =========================================================================
    // 1. TRANSPORTE
    // =========================================================================
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
    ReaperActionDef {
        subsystem: Subsystem::Transport,
        action_id: 40043,
        name: "Transport: Go to end of project",
        canonical_es: "ir al final",
        variations: &[
            "anda al final del proyecto",
            "salta al fin del tema",
            "ir hasta el final",
            "al final",
        ],
    },

    // =========================================================================
    // 2. PISTAS (TRACKS)
    // =========================================================================
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
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 40285,
        name: "Track: Go to next track",
        canonical_es: "siguiente pista",
        variations: &[
            "bajar a la siguiente pista",
            "pista de abajo",
            "proxima pista",
            "avanza de pista",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 40286,
        name: "Track: Go to previous track",
        canonical_es: "pista anterior",
        variations: &[
            "subir a la pista anterior",
            "pista de arriba",
            "pista previa",
            "volver a la pista anterior",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 40340,
        name: "Track: Unsolo all tracks",
        canonical_es: "quitar todos los solos",
        variations: &[
            "desactiva todos los solos",
            "desola todo",
            "sacar solo a todas las pistas",
            "limpiar solos",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Track,
        action_id: 40339,
        name: "Track: Unmute all tracks",
        canonical_es: "desmutear todas las pistas",
        variations: &[
            "quita el mute a todo",
            "desmutea todo",
            "activa el audio de todas las pistas",
            "limpiar mutes",
        ],
    },

    // =========================================================================
    // 3. ITEMS Y EDICIÓN (OSARA & ACCESSIBILITY WIKI STANDARDS)
    // =========================================================================
    ReaperActionDef {
        subsystem: Subsystem::Item,
        action_id: 40012,
        name: "Item: Split items at edit or play cursor",
        canonical_es: "cortar item",
        variations: &[
            "corta el item",
            "divide el item aca",
            "hacer un split",
            "corta en el cursor",
            "separar item",
            "partir el audio",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Item,
        action_id: 40184,
        name: "Item: Duplicate items",
        canonical_es: "duplicar item",
        variations: &[
            "duplica los items seleccionados",
            "clona este item",
            "copiar y pegar item adelante",
            "duplica el audio",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Item,
        action_id: 40006,
        name: "Item: Remove items",
        canonical_es: "eliminar item",
        variations: &[
            "borra el item seleccionado",
            "elimina este audio",
            "quitar item",
            "borra los items",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Item,
        action_id: 40108,
        name: "Item properties: Normalize items",
        canonical_es: "normalizar item",
        variations: &[
            "normaliza el audio",
            "normalizar items seleccionados",
            "ajusta ganancia al maximo",
            "normalizar a cero db",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Item,
        action_id: 40034,
        name: "Item grouping: Group items",
        canonical_es: "agrupar items",
        variations: &[
            "agrupa estos items",
            "crea un grupo con los items seleccionados",
            "unir en grupo",
        ],
    },

    // =========================================================================
    // 4. TAKES
    // =========================================================================
    ReaperActionDef {
        subsystem: Subsystem::Take,
        action_id: 40125,
        name: "Take: Switch items to next take",
        canonical_es: "siguiente toma",
        variations: &[
            "cambia a la siguiente toma",
            "proxima toma",
            "siguiente take",
            "pasa al proximo take",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Take,
        action_id: 40126,
        name: "Take: Switch items to previous take",
        canonical_es: "toma anterior",
        variations: &[
            "volver a la toma anterior",
            "toma previa",
            "take anterior",
            "retrocede al take previo",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Take,
        action_id: 40131,
        name: "Take: Crop to active take in items",
        canonical_es: "conservar toma activa",
        variations: &[
            "borra las demas tomas",
            "deja solo esta toma",
            "crop to active take",
            "elimina tomas no usadas",
        ],
    },

    // =========================================================================
    // 5. SELECCIÓN DE TIEMPO Y CURSOR
    // =========================================================================
    ReaperActionDef {
        subsystem: Subsystem::TimeSelection,
        action_id: 40625,
        name: "Time selection: Set start point",
        canonical_es: "marcar inicio de seleccion",
        variations: &[
            "inicio de la seleccion aca",
            "punto de entrada",
            "set in point",
            "comienzo del loop aca",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::TimeSelection,
        action_id: 40626,
        name: "Time selection: Set end point",
        canonical_es: "marcar final de seleccion",
        variations: &[
            "final de la seleccion aca",
            "punto de salida",
            "set out point",
            "fin del loop aca",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::TimeSelection,
        action_id: 40020,
        name: "Time selection: Remove time selection",
        canonical_es: "quitar seleccion de tiempo",
        variations: &[
            "borra la seleccion de tiempo",
            "deseleccionar tiempo",
            "sacar seleccion",
            "limpiar loop",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Navigation,
        action_id: 40104,
        name: "View: Move edit cursor forward one measure",
        canonical_es: "avanzar un compas",
        variations: &[
            "avanza un compas",
            "siguiente compas",
            "adelanta un compas",
            "un compas adelante",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Navigation,
        action_id: 40105,
        name: "View: Move edit cursor back one measure",
        canonical_es: "retroceder un compas",
        variations: &[
            "retrocede un compas",
            "compas anterior",
            "un compas atras",
            "vuelve un compas",
        ],
    },

    // =========================================================================
    // 6. ENVELOPES Y AUTOMATIZACIÓN
    // =========================================================================
    ReaperActionDef {
        subsystem: Subsystem::Envelope,
        action_id: 40061,
        name: "Track: Toggle volume envelope visible",
        canonical_es: "mostrar envolvente de volumen",
        variations: &[
            "muestra el volumen de la pista",
            "ver curva de volumen",
            "mostrar automatizacion de volumen",
            "abrir volumen",
        ],
    },
    ReaperActionDef {
        subsystem: Subsystem::Envelope,
        action_id: 40062,
        name: "Track: Toggle pan envelope visible",
        canonical_es: "mostrar envolvente de paneo",
        variations: &[
            "muestra el paneo de la pista",
            "ver curva de paneo",
            "mostrar automatizacion de paneo",
            "abrir paneo",
        ],
    },

    // =========================================================================
    // 7. MARCADORES
    // =========================================================================
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

    // =========================================================================
    // 8. OPCIONES Y METRÓNOMO
    // =========================================================================
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
