Sumar cada rango día a día puede costar hasta n pasos por consulta. Con muchas consultas largas, es demasiado lento.
---
Si conocieras la lluvia total desde el día 0 hasta cualquier día, ¿podrías obtener el total de un rango con una sola resta?
---
Construye `prefix` con `prefix[0] = 0` y `prefix[d + 1] = prefix[d] + rain[d]`. La respuesta a `[first, last]` es `prefix[last + 1] - prefix[first]`.
