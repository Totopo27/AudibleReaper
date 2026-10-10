---
name: reaper-audio-daemon
description: "Motor daemon de captura de audio, Push-to-Talk (teclado y MIDI), reproduccion de earcons acusticos via WASAPI y cliente IPC."
---

# reaper-audio-daemon

## Responsabilidad
Gestionar el ciclo de vida del audio en tiempo real fuera de REAPER:
1. Captura del micrófono mediante WASAPI / CPAL en Rust al recibir la señal de Push-to-Talk.
2. Detección de atajos de teclado globales (Windows API hooks) y eventos de entrada MIDI (pedal de sustain, pads o botones CC vía `midir`).
3. Reproducción inmediata de Earcons de retroalimentación acústica (`PTT_START`, `PTT_STOP`, `CMD_OK`, `CMD_ERROR`) en canal WASAPI dedicado de auriculares para no videntes.
4. Conexión como cliente al Named Pipe `\\.\\pipe\\reaperkev_ipc` para enviar las acciones resueltas.

## Principios
1. **Prioridad en Monitoreo**: Los earcons acústicos deben reproducirse con <15ms de latencia tras soltar o activar el PTT.
2. **Aislamiento Acústico**: El dispositivo de salida de los earcons no debe cruzarse con el bus de salida master de REAPER para evitar grabaciones fantasma.
