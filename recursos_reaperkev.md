# Recursos y Documentación Oficial: reaperKev

## 1. REAPER & SDKs Oficiales
- **REAPER Extension SDK (C++)**: [https://www.reaper.fm/sdk/plugin/plugin.php](https://www.reaper.fm/sdk/plugin/plugin.php)
  - Repositorio oficial Cockos: [https://github.com/justinfrankel/reaper-sdk](https://github.com/justinfrankel/reaper-sdk)
  - `reaper_plugin.h`: Define el punto de entrada DLL `REAPER_PLUGIN_ENTRYPOINT` (`ReaperPluginEntry`), el struct `reaper_plugin_info_t` y el sistema `Register("timer", void (*timer_func)())` y `Register("hookcommand", ...)`.
  - `reaper_plugin_functions.h`: Contiene las 800+ funciones de la API C nativa de Cockos (`Main_OnCommand`, `GetTrack`, `TrackFX_GetByName`, `Undo_BeginBlock`, etc.), cargadas dinámicamente vía `REAPERAPI_LoadAPI(rec->GetFunc)`.
  - Requisito de ABI en Windows: MSVC toolchain (`x86_64-pc-windows-msvc`). Los binarios deben llamarse con prefijo `reaper_` (ej. `reaper_kev.dll`) y ubicarse en `<REAPER_RESOURCE_PATH>/UserPlugins/`.
- **ReaScript Documentation**: [https://www.reaper.fm/sdk/reascript/reascript.php](https://www.reaper.fm/sdk/reascript/reascript.php)
  - Catálogo de funciones de control de transporte, pistas, items, marcadores y FX.
- **WDL (Cockos White Dynamic Link library)**: [https://www.cockos.com/wdl/](https://www.cockos.com/wdl/)
  - Librería base C++ de Cockos que incluye estructuras de datos de alto rendimiento y utilidades cross-platform.
- **reaper-rs (Rust bindings for REAPER C API)**: [https://github.com/helgoboss/reaper-rs](https://github.com/helgoboss/reaper-rs)
  - Creado por Benjamin Klum (autor de ReaLearn y Playtime). Expone:
    - `reaper-low`: Bindings directos autogenerados contra `reaper_plugin_functions.h`.
    - `reaper-medium`: API idiomática y segura en Rust con wrappers de `ReaperSession`, registro de acciones y control surfaces.
    - `reaper-macros`: Macro `#[reaper_extension_plugin]` para inicializar `reaper_kev.dll` sin boilerplate.

## 2. Accesibilidad & Comunidad
- **OSARA (Open Source Accessibility for the REAPER Application)**: [https://github.com/jcsteh/osara](https://github.com/jcsteh/osara)
  - Estándar de accesibilidad de código abierto para personas ciegas en REAPER, integrando NVDA, JAWS y VoiceOver.
  - Documentación de cómo interactúa el cursor de edición y navegación de tracks sin mouse.
- **The Reaper Accessibility Wiki**: [https://www.reaperaccessibility.com/wiki/Main_Page](https://www.reaperaccessibility.com/wiki/Main_Page)
  - La enciclopedia oficial mantenida por la comunidad global de usuarios ciegos de REAPER.
  - **Reaper shortcut key list by headings**: [https://www.reaperaccessibility.com/wiki/Reaper_shortcut_key_list_by_headings](https://www.reaperaccessibility.com/wiki/Reaper_shortcut_key_list_by_headings)
    - Mapeo canónico exhaustivo de atajos de OSARA + SWS organizados por categorías: Transporte, Pistas, Takes, Envelopes/Automatización, Selección de tiempo, Marcadores/Regiones, MIDI Editor y Peak Watcher.
    - Este catálogo constituye la base directa para la taxonomía de intenciones del modelo Kev.
  - **Reaper and Osara Cookbook**: Casos de uso de producción real y resolución de flujos de trabajo sin pantalla.
- **SWS / S&M Extension**: [https://www.sws-extension.org/](https://www.sws-extension.org/)
  - Repositorio: [https://github.com/reaper-oss/sws](https://github.com/reaper-oss/sws)
  - Extensión comunitaria en C++ de referencia para arquitectura de plugins avanzados en REAPER.

## 3. Audio & Captura Nativa (Windows)
- **CPAL (Cross-Platform Audio Library in Rust)**: [https://github.com/RustAudio/cpal](https://github.com/RustAudio/cpal)
  - Captura y reproducción nativa de baja latencia vía Windows WASAPI (probado en zenKev).
- **Windows Named Pipes API (IPC)**:
  - `CreateNamedPipeW`, `ConnectNamedPipe`, `ReadFile`, `WriteFile` en `kernel32.dll`.
  - Latencia sub-milisegundo (< 1ms) con buffers asíncronos (`FILE_FLAG_OVERLAPPED`).
- **Windows MIDI API / midir (Rust)**: [https://github.com/Boddlnagg/midir](https://github.com/Boddlnagg/midir)
  - Monitoreo directo de eventos MIDI CC y Note-On para activación Push-to-Talk vía pedales o controladores externos.

## 4. Modelos de Lenguaje & STT (Kev Engine)
- **Whisper.cpp / Moonshine ONNX**:
  - Motores locales de inferencia de voz optimizados para CPU/CUDA con transcripción rápida.
- **Kev SLM (Small Language Model fine-tuning)**:
  - Repositorio base y técnicas de curaduría probadas en zenKev (`zenkev-v6-strict`).
  - Mapeo directo de entrada transcrita a tuplas JSON: `{"subsystem": string, "action_id": number, "args": object}`.

## 5. Canales de Consulta y Resolución Técnica
- **Foro Oficial de Cockos**: [https://forum.cockos.com/](https://forum.cockos.com/) (Secciones: *ReaScript, JSFX, REAPER Plug-in Extensions*, *Developer*).
- **Cockos WDL / SWELL Git**: [https://github.com/justinfrankel/WDL](https://github.com/justinfrankel/WDL)
- **Context7 MCP**: Para consultas y resolución de APIs actualizadas de crates Rust (`cpal`, `midir`, `tokio`, etc.).
