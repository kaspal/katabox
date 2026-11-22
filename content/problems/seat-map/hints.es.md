Con una sola fila el plano siempre sale bien. Prueba con dos filas y una sola reserva: ¿la otra fila sigue libre?
---
El bucle que marca las reservas toca exactamente una casilla. Mira cómo se crea la sala antes del bucle: ¿cuántas listas de fila distintas existen de verdad?
---
`[["."] * cols] * rows` (y `fill` con un único array en TypeScript) pone la misma lista de fila en todas las posiciones, así que marcar una fila las marca todas. Crea una lista nueva para cada fila: `[["."] * cols for _ in range(rows)]`.
