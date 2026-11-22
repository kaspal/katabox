Un árbol de Fenwick que actualiza un estante cada vez necesitaría hasta 10⁵
actualizaciones por entrega. ¿Se puede registrar una entrega a toda una fila con un número
constante de actualizaciones puntuales?
---
Registra una entrega como `+count` en el estante `a` y `−count` justo después del estante
`b` en un array de diferencias: la suma prefija de ese array en el estante `i` es cuánto
recibió el estante `i`. Ahora necesitas sumas de esas sumas prefijas.
---
El total de los estantes `0..i−1` es `i · D(i) − W(i)`, donde `D(i)` es la suma de las
diferencias en posiciones menores que `i` y `W(i)` es la suma de cada diferencia por su
posición. Guarda dos árboles de Fenwick, uno para cada una, y responde `total(a, b)` como
`prefix(b + 1) − prefix(a)`.
