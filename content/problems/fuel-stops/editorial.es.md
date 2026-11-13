## Enfoque: repostar a posteriori en la mayor gasolinera pasada

Sea `reach` hasta dónde puede llegar la furgoneta con las paradas elegidas hasta ahora (empieza en `fuel`). Recorre las gasolineras en orden:

- mientras la posición de la gasolinera quede más allá de `reach`, la furgoneta tuvo que parar en alguna gasolinera anterior: toma la que tiene **más litros** entre las pasadas y aún no usadas (un montículo de máximos), suma sus litros a `reach` y cuenta una parada; si no hay ninguna, devuelve −1;
- después mete los litros de esta gasolinera en el montículo.

Al final, haz lo mismo hasta que `reach ≥ target`.

**Por qué la mayor:** cada vez que hace falta más combustible, usar la mayor gasolinera disponible amplía el alcance lo más posible con una sola parada, y no importa cuáles se usaron antes, solo cuántas y cuánto dieron.

**Complejidad:** O(n log n) en tiempo y O(n) de espacio. El DP `lejos[t]` = alcance máximo con t paradas, actualizado para cada gasolinera sobre cada t, es O(n²).
