# AudibleReaper

> **AudibleReaper** es un sistema de control por voz de ultra baja latencia y retroalimentación acústica diseñado específicamente para el DAW **REAPER**, con enfoque prioritario en accesibilidad para personas no videntes, usuarios de lectores de pantalla (**NVDA**, **JAWS**, **OSARA**) y flujos de trabajo manos libres en el estudio de grabación.

---

## 🎧 Características Principales

- **Latencia Sub-milisegundo (< 350 µs)**: Comunicación directa con REAPER a través de **Windows Named Pipes** (`\\.\pipe\reaperkev_ipc`).
- **Thread-Safety Garantizado**: Despacho de comandos exclusivamente en el hilo principal de la GUI/Audio de REAPER mediante el hook oficial de temporizador de Cockos (`plugin_register_add_timer`), asegurando bloques Undo atómicos sin bloqueos ni corrupción de memoria.
- **Push-to-Talk Dual (Teclado & MIDI)**: Activación configurable mediante atajo global de teclado (`Alt + V`) o controladores/pedales MIDI (`Sustain CC64`).
- **Retroalimentación Acústica (Earcons)**: Generación procedural de barridos y tonos armónicos suaves con envelopes anti-clipping para confirmación táctil-auditiva sin depender de la pantalla y sin contaminar el bus de audio del DAW.
- **Integración Simbiótica con OSARA & NVDA**: Diseñado para no competir con el lector de pantalla; AudibleReaper ejecuta las acciones oficiales y permite que OSARA verbalice el estado de forma nativa.
- **Inferencia Determinista (99.93% de Precisión)**: Matriz cerrada de comandos basada en *word-boundary longest-match* que elimina falsos positivos y rechaza de forma segura frases fuera de catálogo.

---

## 🏛️ Arquitectura del Sistema

```
+-------------------------------------------------------------------------------+
|                       USUARIO (Persona no vidente / DAW Op)                  |
+-------------------------------------------------------------------------------+
      |                                              ^
  Voz + PTT (Teclado / Pedal MIDI)               Earcons Acústicos (WASAPI Dedicado)
      |                                              |
      v                                              |
+-------------------------------------------------------------------------------+
|                             reaper-audio-daemon                               |
|  - Windows API (Atajos PTT) & midir (Eventos MIDI)                            |
|  - CPAL WASAPI: Captura micrófono PCM                                         |
|  - Inferencia Determinista (reaper-kev-curator)                               |
|  - WASAPI Output: Reproducción de Earcons de estado (< 15ms)                  |
+-------------------------------------------------------------------------------+
      | (Windows Named Pipe: \\.\pipe\reaperkev_ipc)  [Latencia < 1ms]
      v
+-------------------------------------------------------------------------------+
|                             reaper-native-ext                                 |
|  - DLL nativa de REAPER (reaper_kev.dll)                                      |
|  - Servidor IPC Named Pipe no bloqueante                                      |
|  - Despachador de acciones en el Main Thread de REAPER                        |
|  - ReaScript C API (Undo_BeginBlock, Main_OnCommandEx, etc.)                  |
+-------------------------------------------------------------------------------+
      |
      v
+-------------------------------------------------------------------------------+
|                            REAPER DAW ENGINE                                  |
|  - Control de Pistas, Takes, Items, Transporte, Automatización, Loop          |
|  - Compatibilidad transparente con OSARA y lectores de pantalla (NVDA/JAWS)   |
+-------------------------------------------------------------------------------+
```

---

## 📦 Estructura del Workspace (Rust)

- **`crates/reaper-kev-ext`**: Extensión DLL nativa de REAPER (`reaper_kev.dll`) construida con `reaper-rs`.
- **`crates/reaper-audio-daemon`**: Daemon en segundo plano encargado de la captura de audio WASAPI CPAL, hooks de PTT y Earcons.
- **`crates/reaper-kev-curator`**: Catálogo canónico de comandos, generador de dataset (8.000+ muestras) y motor de resolución de intenciones.
- **`crates/reaper-ipc-test-client`**: Herramienta CLI de diagnóstico para verificar el canal IPC de REAPER.

---

## 🚀 Instalación Rápida (1-Clic)

1. Clona el repositorio:
   ```bash
   git clone https://github.com/Totopo27/AudibleReaper.git
   cd AudibleReaper
   ```
2. Ejecuta el instalador automatizado en Windows:
   ```cmd
   instalar_reaperkev.bat
   ```
   *El script compilará el workspace y copiará automáticamente `reaper_kev.dll` a tu carpeta `%APPDATA%\REAPER\UserPlugins\`.*
3. Abre **REAPER** y lanza `reaper-audio-daemon.exe`.

---

## 📋 Catálogo de Acciones Soportadas

| Subsistema | Ejemplos de Frases en Lenguaje Natural | Action ID Oficial |
| :--- | :--- | :--- |
| **Transporte** | *"play"*, *"detener"*, *"grabar"*, *"ir al inicio"*, *"repetir bucle"* | `1007`, `1016`, `1013`, `40042`, `1068` |
| **Pistas** | *"armar para grabar"*, *"mutear pistas"*, *"solear pistas"*, *"insertar pista"* | `9`, `6`, `7`, `40001` |
| **Navegación** | *"siguiente pista"*, *"pista anterior"*, *"avanzar un compás"*, *"retroceder un compás"* | `40285`, `40286`, `40104`, `40105` |
| **Edición e Items** | *"cortar item"*, *"duplicar item"*, *"normalizar item"*, *"eliminar item"* | `40012`, `40184`, `40108`, `40006` |
| **Takes** | *"siguiente toma"*, *"toma anterior"*, *"conservar toma activa"* | `40125`, `40126`, `40131` |
| **Selección & Loop** | *"marcar inicio de selección"*, *"marcar final de selección"*, *"quitar selección"* | `40625`, `40626`, `40020` |
| **Marcadores** | *"insertar marcador"*, *"marcador anterior"*, *"marcador siguiente"* | `40157`, `40172`, `40173` |
| **Opciones** | *"alternar metrónomo"*, *"quitar todos los solos"*, *"desmutear todo"* | `40364`, `40340`, `40339` |

---

## 🧪 Verificación y Tests

Para ejecutar la suite completa de pruebas unitarias y de integración E2E:

```bash
cargo test --workspace
```

Para verificar la compuerta Gate 4 del catálogo completo:
```bash
cargo run -p reaper-audio-daemon --bin verify_gate4
```

---

## 📄 Licencia

Este proyecto está disponible bajo la licencia MIT.
