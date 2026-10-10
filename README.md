# AudibleReaper

Control por voz para REAPER pensado para operar sin mirar el monitor.

Surgió de un problema concreto: cuando estás grabando una guitarra o tocando el teclado, soltar las manos para buscar el mouse o memorizar combinaciones de tres teclas rompe la sesión. En el caso de personas ciegas o con baja visión que usan REAPER a diario con lectores de pantalla (NVDA, JAWS) y la extensión OSARA, la interfaz es rápida con el teclado, pero ciertas tareas rutinarias se vuelven lentas o requieren demasiados pasos seguidos.

AudibleReaper resuelve esto agregando un canal de voz directo y retroalimentación auditiva inmediata, sin pelearse con el lector de pantalla.

---

## Cómo encaja con OSARA y NVDA

Un error común al armar herramientas asistivas es meter un sintetizador de voz que intente leer todo en voz alta. Si hacés eso dentro de REAPER, tu voz artificial empieza a hablar encima de NVDA y no entendés ni la música ni el lector.

AudibleReaper toma el camino opuesto:
1. **Acción directa**: Cuando decís *"armar para grabar"* o *"siguiente pista"*, la extensión dispara el Action ID nativo de Cockos o SWS directamente en el hilo principal de REAPER.
2. **OSARA se encarga de hablar**: Como la acción ocurre de forma nativa en REAPER, OSARA la detecta por UIA/MSAA y le avisa a NVDA al instante (*"Pista 2: Guitarras, armada"*).
3. **Earcons acústicos sutiles**: AudibleReaper solo emite tonos cortos y suaves para indicar el estado del micrófono (abierto, cerrado, comando ejecutado o error), a través de un canal de audio independiente para no contaminar tus tomas de grabación.

---

## Cómo funciona por dentro

La herramienta se divide en dos componentes comunicados por un Named Pipe local en Windows (`\\.\pipe\reaperkev_ipc`):

- **`reaper_kev.dll` (Extensión C++/Rust)**: Se carga en la carpeta `UserPlugins` de REAPER. Escucha comandos en segundo plano y los despacha al motor de REAPER usando el hook de temporizador oficial (`plugin_register_add_timer`). Esto asegura que cada acción corra en el hilo principal de la interfaz y quede guardada en el historial de Deshacer (Undo).
- **`reaper-audio-daemon.exe` (Servicio en segundo plano)**: Monitorea el micrófono con WASAPI (vía CPAL) únicamente mientras mantenés presionado el atajo de Push-to-Talk (por defecto `Alt + V` o el pedal de Sustain MIDI `CC64`). Al soltarlo, procesa la frase, resuelve la acción y envía la orden por el pipe. Todo el ciclo toma menos de 1 milisegundo.

---

## Instalación rápida (Windows)

1. Tené REAPER instalado en tu máquina.
2. Cloná el repositorio:
   ```bash
   git clone https://github.com/Totopo27/AudibleReaper.git
   cd AudibleReaper
   ```
3. Ejecutá el instalador por lotes:
   ```cmd
   instalar_reaperkev.bat
   ```
   El script compila el código, detecta tu carpeta `%APPDATA%\REAPER\UserPlugins\` y copia la DLL en su lugar.
4. Abrí REAPER.
5. Iniciá `reaper-audio-daemon.exe` (el instalador te pregunta si querés abrirlo de una vez).

---

## Comandos cotidianos que podés probar

El modelo trabaja con una matriz cerrada para evitar que interprete cualquier ruido como una orden. Podés hablarle en español rioplatense o neutro:

- **Transporte**: *"dale play"*, *"detener"*, *"grabá"*, *"ir al inicio"*, *"repetir bucle"*.
- **Pistas**: *"armar para grabar"*, *"muteá la pista"*, *"soleá este canal"*, *"insertar pista nueva"*, *"siguiente pista"*, *"pista anterior"*, *"quitar todos los solos"*.
- **Edición y cortes**: *"cortar item"* (hace split en el cursor), *"duplicar item"*, *"normalizar a cero dB"*, *"borrar este audio"*.
- **Tomas**: *"siguiente toma"*, *"toma anterior"*, *"dejar solo esta toma"* (crop to active take).
- **Compases y tiempo**: *"avanzar un compás"*, *"retroceder un compás"*, *"marcar inicio de selección"*, *"marcar final de selección"*, *"limpiar selección"*.
- **Marcadores**: *"insertar marcador acá"*, *"marcador siguiente"*, *"marcador anterior"*.
- **Metrónomo**: *"prender el metrónomo"*, *"apagar el click"*.

Si decís algo que no tiene nada que ver con REAPER (*"haceme un café"*), el sistema emite un tono grave de descarte y no toca nada en tu proyecto.

---

## Verificación y pruebas

Si querés auditar el código o correr las pruebas del workspace:

```bash
# Compilar y correr pruebas unitarias e integración IPC
cargo test --workspace

# Ejecutar la verificación completa de catálogo (70 pruebas con medición de latencia)
cargo run -p reaper-audio-daemon --bin verify_gate4
```

---

## Licencia

Código publicado bajo licencia MIT. Las contribuciones, mejoras en diccionarios de voz y adaptaciones para controladores MIDI son bienvenidas.
