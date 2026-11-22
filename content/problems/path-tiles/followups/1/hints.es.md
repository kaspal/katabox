La recurrencia del problema base, `f(i) = f(i − 1) + f(i − 2) + f(i − 3)`, sigue siendo
correcta, pero un bucle de 10¹⁵ pasos no termina nunca. ¿Cómo puedes dar muchos pasos de
golpe?
---
Un paso convierte la terna `(f(i), f(i − 1), f(i − 2))` en `(f(i + 1), f(i), f(i − 1))`, y
ese paso es siempre la misma aplicación lineal: una matriz fija de 3×3.
---
Dar `n` pasos es elevar esa matriz a la `n`. Calcúlala elevando al cuadrado repetidamente
(eleva la matriz al cuadrado y multiplícala en el resultado cuando el bit actual de `n`
vale 1), reduciendo cada entrada módulo 1 000 003, y aplícala después a
`(f(0), f(−1), f(−2)) = (1, 0, 0)`.
