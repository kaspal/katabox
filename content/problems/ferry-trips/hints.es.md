El excursionista más pesado tiene que cruzar en algún viaje. ¿Quién es su mejor acompañante, si lo hay?
---
Si el más pesado no puede ir con el más ligero, no puede ir con nadie. Si puede, emparejarlo con el más ligero nunca empeora la solución.
---
Ordena los pesos. Mantén `lo` en el más ligero y `hi` en el más pesado: cada viaje lleva a `hi` (y `hi -= 1`), y también a `lo` cuando `weights[lo] + weights[hi] ≤ limit` (y `lo += 1`). Cuenta los viajes hasta que los punteros se crucen.
