---
name: reaper-native-ext
description: "Desarrollo y mantenimiento de la extension nativa de REAPER (reaper_kev.dll) y el servidor IPC Named Pipes."
---

# reaper-native-ext

## Responsabilidad
Responsable exclusivo de la extensión binaria nativa de REAPER (`reaper_kev.dll`), la conexión con el REAPER Extension SDK (`reaper_plugin.h`), el servidor IPC multihilo en Windows (`\\.\\pipe\\reaperkev_ipc`) y la ejecución segura de comandos de REAPER en el hilo principal de la GUI/Audio.

## Principios
1. **Thread-Safety en REAPER**: Nunca llamar funciones que modifiquen el estado del proyecto (`Undo_BeginBlock`, `Main_OnCommand`, `SetMediaTrackInfo_Value`) fuera del hilo principal de REAPER. Usar un despachador encolado o hook de temporizador/bucle de mensajes.
2. **IPC No Bloqueante**: El servidor Named Pipe opera en un hilo separado o en modo asíncrono sobrepuesto (`FILE_FLAG_OVERLAPPED`).
3. **Manejo de Undo Atómico**: Toda acción ejecutada por lote debe quedar encapsulada en un bloque `Undo_BeginBlock` / `Undo_EndBlock` con descripción clara.
