# Plan de Trabajo y Compuertas de Validación (Gates): reaperKev

## 1. Diagrama de Arquitectura en Capas

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
|  - STT Engine (Moonshine / Whisper.cpp) -> Transcripción                      |
|  - WASAPI Output: Reproducción de Earcons de estado (< 15ms)                  |
+-------------------------------------------------------------------------------+
      | (Texto transcrito)
      v
+-------------------------------------------------------------------------------+
|                             reaper-kev-curator                                |
|  - Modelo Kev (SLM cuantizado fine-tuned)                                     |
|  - Inferencia de Intenciones -> Mapeo a Matriz de Action IDs de Cockos/SWS    |
|  - Payload JSON: {"command_id": 1007, "args": {...}, "track": 2}              |
+-------------------------------------------------------------------------------+
      | (Windows Named Pipe: \\.\pipe\reaperkev_ipc)  [Latencia < 1ms]
      v
+-------------------------------------------------------------------------------+
|                             reaper-native-ext                                 |
|  - DLL nativa de REAPER (reaper_kev.dll)                                      |
|  - Servidor IPC Named Pipe no bloqueante                                      |
|  - Despachador de acciones en el Main Thread de REAPER                        |
|  - ReaScript C API (Undo_BeginBlock, Main_OnCommand, etc.)                     |
+-------------------------------------------------------------------------------+
      |
      v
+-------------------------------------------------------------------------------+
|                            REAPER DAW ENGINE                                  |
|  - Control de Pistas, FX, Transporte, Grabación                               |
|  - Compatibilidad transparente con OSARA y lectores de pantalla (NVDA/JAWS)   |
+-------------------------------------------------------------------------------+
```

---

## 2. Ecosistema de Skills Locales

| Skill | Rol / Dominio | Entregable Principal |
| :--- | :--- | :--- |
| `reaper-native-ext` | Bajo nivel REAPER / C++ / Rust | Extensión `reaper_kev.dll` y servidor Named Pipe. |
| `reaper-audio-daemon` | Captura, PTT y Audio Feedback | Daemon de captura de voz, triggers MIDI/Teclado y Earcons WASAPI. |
| `reaper-kev-curator` | IA / NLP / Fine-tuning | Dataset cerrado de intenciones, entrenamiento y pesas del modelo Kev. |

---

## 3. Fases Secuenciales y Compuertas de Validación (Gates)

### Fase 0: Entorno, Recursos y Esqueleto del IPC (Tracer Bullet)
- **Objetivo**: Establecer la comunicación de prueba entre un cliente de consola externo y la extensión dentro de REAPER.
- **Actividades**:
  1. Configurar proyecto de la extensión nativa (`reaper_kev.dll`).
  2. Implementar servidor Named Pipe `\\.\pipe\reaperkev_ipc`.
  3. Crear un script/cliente de prueba para enviar un comando básico (ej. Action ID 40044: Play/Stop).
- **Compuerta de Salida (Gate 0)**:
  - *Binario medible*: Enviar payload `{"action_id": 40044}` por el pipe desde terminal externa y verificar que REAPER inicie/detenga la reproducción en menos de 5 ms sin congelar la interfaz.

### Fase 1: Audio Daemon, Push-to-Talk y Earcons Acústicos
- **Objetivo**: Disponer de la capa física de interacción sin pantalla.
- **Actividades**:
  1. Implementar captura WASAPI con CPAL activada por PTT de teclado.
  2. Implementar captura de evento MIDI (ej. Pedal Sustain CC64) para activar PTT.
  3. Integrar reproductor de Earcons de baja latencia en el daemon.
- **Compuerta de Salida (Gate 1)**:
  - *Binario medible*: Presionar y soltar el atajo/pedal MIDI 20 veces consecutivas; el daemon debe reproducir el earcon de inicio/cierre con latencia < 20 ms sin desbordar el búfer de audio.

### Fase 2: Dataset Inicial de Transporte y Pistas (Kev Fase 1)
- **Objetivo**: Curar la matriz cerrada de comandos para operaciones esenciales de estudio.
- **Actividades**:
  1. Mapear catálogo de acciones: Transporte (Play, Stop, Record, Loop), Pistas (Crear, Armar, Mute, Solo, Volumen) y Marcadores.
  2. Generar dataset sintético y balanceado (mínimo 1.000 ejemplos anotados en español).
  3. Entrenar/afinar el adaptador o modelo Kev para este catálogo.
- **Compuerta de Salida (Gate 2)**:
  - *Binario medible*: Batería de evaluación dev set con precisión en `action_id` >= 95% y detección de comandos fuera de dominio (rechazo seguro) = 100%.

### Fase 3: Integración E2E Tracer Bullet (Voz -> DAW)
- **Objetivo**: Primer control por voz de extremo a extremo en REAPER.
- **Actividades**:
  1. Conectar STT (Moonshine / Whisper) + Kev con el cliente del Named Pipe.
  2. Probar flujo continuo: PTT -> Hablar ("Armá la pista dos para grabar") -> Earcon OK -> Pista armada en REAPER.
- **Compuerta de Salida (Gate 3)**:
  - *Binario medible*: Latencia total fin a fin (fin de voz a acción en REAPER) < 400 ms en 10 pruebas consecutivas sin errores.

### Fase 4: Expansión de Bloques (Edición, FX y Parámetros)
- **Objetivo**: Extender el catálogo de comandos a inserción de FX, control de faders y edición de items.
- **Actividades**:
  1. Integración de parámetros flotantes ("subí 3 dB el volumen").
  2. Compatibilidad con extensiones SWS.
- **Compuerta de Salida (Gate 4)**:
  - *Binario medible*: Batería completa de 50 comandos diversos ejecutados en un proyecto real con OSARA activo sin interferencias de audio.
