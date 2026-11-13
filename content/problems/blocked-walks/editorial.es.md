## Enfoque: inclusión–exclusión sobre el primer cruce cortado

El número de paseos de `(a, b)` a `(c, d)` con `a ≤ c`, `b ≤ d` es `caminos = C((c − a) + (d − b), c − a)`. Con factoriales e inversos de factoriales hasta `rows + cols` (2·10⁵ < p, así que todos los inversos existen), cada `caminos` es O(1).

Ordena los cruces cortados por fila y luego por columna, y añade el destino al final. A un cruce solo se puede llegar desde cruces anteriores en este orden. Define `good[i]` como los paseos desde la salida hasta el punto `i` que no tocan ningún cruce cortado antes de él:

`good[i] = caminos(salida → i) − Σ sobre j < i con r_j ≤ r_i y c_j ≤ c_i de good[j] · caminos(j → i)`.

Cada paseo hasta `i` que toca un cruce cortado tiene un **primero**, `j`, y se cuenta exactamente una vez en la suma (su parte hasta `j` es uno de los `good[j]` paseos, y el resto es cualquier paseo de `j` a `i`). La respuesta es `good` del destino.

### Paso a paso

Ejemplo 1: el cruce cortado (1, 1) tiene `good = caminos((0,0) → (1,1)) = 2`. El destino tiene `caminos = 6`, menos `2 · caminos((1,1) → (2,2)) = 2 · 2`: 6 − 4 = 2.

**Complejidad:** O(rows + cols + k²) en tiempo con k ≤ 1000 cruces cortados. El DP sobre la cuadrícula es O(rows · cols), 10¹⁰ pasos.
