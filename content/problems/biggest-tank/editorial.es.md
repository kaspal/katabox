## Enfoque: estrechar por el lado más bajo

Pon `i` en la primera tabla y `j` en la última. Anota `(j − i) · min(boards[i], boards[j])`. Después mueve un paso hacia dentro el puntero de la tabla **más baja**, y repite hasta que `i = j`.

**Por qué es seguro descartar la tabla baja:** supón que `boards[i] ≤ boards[j]`. Cualquier otro depósito que use la tabla `i` la empareja con alguna tabla `k` entre `i` y `j`; es más estrecho que `j − i` y su altura es como mucho `boards[i]`. Así que ninguno supera al depósito recién anotado, y la tabla `i` se puede descartar. Cada paso descarta una tabla, así que tras n − 1 pasos cada tabla se ha considerado con su mejor pareja.

### Paso a paso

`[2, 7, 4, 1, 8, 3, 6]`: (0, 6) da 6 · 2 = 12, se descarta la tabla 0; (1, 6) da 5 · 6 = 30, se descarta la 6; (1, 5) da 4 · 3 = 12, se descarta la 5; (1, 4) da 3 · 7 = 21, se descarta la 1; el resto son menores. La respuesta es 30.

**Complejidad:** O(n) en tiempo y O(1) de espacio. Comprobar cada par es O(n²).
