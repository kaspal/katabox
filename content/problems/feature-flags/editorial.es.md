## Enfoque: sumas sobre superconjuntos en las 2¹⁶ máscaras

Un usuario responde "sí" a la pregunta `q` cuando `usuario & q == q`. En vez de probar cada pareja, precalcula la respuesta para cada pregunta posible.

1. `cuenta[m]` = el número de usuarios cuya configuración es exactamente `m`.
2. Para cada bit `b` (de 0 a 15), para cada máscara `m` que **no** tiene el bit `b`: `cuenta[m] += cuenta[m | b]`.
3. Responde cada pregunta con `cuenta[q]`.

**Por qué funciona el paso 2.** Tras procesar los bits `0 … b`, `cuenta[m]` guarda el número de usuarios cuya configuración es igual a `m` en los bits por encima de `b` y contiene a `m` en los bits `0 … b`. Sumar `cuenta[m | b]` añade los usuarios que tienen el bit `b` cuando `m` no lo pide. Tras el último bit, `cuenta[m]` cuenta todos los superconjuntos de `m`. Es la programación dinámica de "suma sobre superconjuntos".

Solo hace falta procesar los bits que aparecen: con `B` = el bit más alto usado, la tabla tiene 2ᴮ entradas.

### Paso a paso

`users = [5, 7, 2, 15]`: `cuenta[5] = cuenta[7] = cuenta[2] = cuenta[15] = 1`. Tras las sumas, `cuenta[5]` = usuarios ⊇ 0101 = {5, 7, 15} = 3, `cuenta[8]` = {15} = 1, `cuenta[0]` = 4.

**Complejidad:** O(2ᴮ · B + n + q) de tiempo y O(2ᴮ) de memoria, como mucho unos 10⁶ pasos. Comprobar cada usuario para cada pregunta cuesta O(n · q).
