---
name: reaper-kev-curator
description: "Curaduria de datasets, entrenamiento y evaluacion del modelo Kev para REAPER con matriz cerrada de Action IDs."
---

# reaper-kev-curator

## Responsabilidad
Construcción y mantenimiento del pipeline de datos e inferencia del modelo Kev para REAPER:
1. Mapeo exhaustivo del catálogo de acciones nativas de Cockos y extensiones SWS a identificadores canónicos (`subsystem`, `action_id`, `parameters`).
2. Generación sintética y curaduría rigurosa de variaciones en lenguaje natural (español rioplatense, neutro y técnico de audio).
3. Entrenamiento/fine-tuning del modelo Kev SLM (Small Language Model) garantizando cero alucinaciones (zero-leak / closed-matrix).
4. Pruebas de regresión con batería de evaluación acústica y métricas de precisión de asignación de comandos.

## Principios
1. **Determinismo**: Ante una frase ambigua o fuera de catálogo, el modelo debe retornar un estado explícito de descarte (`UNKNOWN_COMMAND` / `CLARIFICATION_REQUIRED`) en vez de ejecutar una acción errónea en el DAW.
2. **Latencia Predecible**: Inferencia con un tiempo límite estricto (< 250ms en CPU/GPU local).
