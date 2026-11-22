Prueba el rango más pequeño posible, con `a` y `b` en el mismo índice. ¿Cuál debería ser la respuesta y qué devuelve el código?
---
Fíjate en los límites del bucle. ¿Qué índices recorre de verdad?
---
`range(a, b)` se detiene antes de `b`, así que `nums[b]` nunca se suma. Recorre hasta `b + 1` (en TypeScript, usa `i <= b`).
