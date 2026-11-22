Prueba un `x` mayor que todos los elementos. ¿Qué índice debería devolverse y puede el código devolverlo alguna vez?
---
El cuerpo del bucle es una búsqueda de cota inferior correcta. Mira el rango con el que empieza.
---
`hi` empieza en `len(sorted) - 1`, así que la respuesta nunca puede ser `len(sorted)`. Empieza con `hi = len(sorted)` (en TypeScript, `sorted.length`).
