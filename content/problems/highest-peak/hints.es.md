Mirando un hito y su vecino de la derecha, ¿puedes saber en qué lado de la cumbre estás?
---
Si `heights[m] < heights[m + 1]`, aún estás subiendo: la cumbre está a la derecha de `m`. Si no, estás en la cumbre o bajando: la cumbre está en `m` o a su izquierda.
---
Búsqueda binaria con `lo = 0`, `hi = n − 1`: mientras `lo < hi`, toma `m = (lo + hi) / 2`; subiendo → `lo = m + 1`, si no `hi = m`. La respuesta es `lo`.
