## Enfoque: sumas prefijas

Calcula los totales acumulados una sola vez: `prefix[0] = 0` y `prefix[d + 1] = prefix[d] + rain[d]`, de modo que `prefix[k]` es la lluvia de los primeros `k` días. La lluvia de `first` a `last` es la de los primeros `last + 1` días menos la de los primeros `first` días: `prefix[last + 1] - prefix[first]`.

### Paso a paso

`rain = [3, 0, 7, 2, 5]` da `prefix = [0, 3, 3, 10, 12, 17]`.

| consulta | cálculo | respuesta |
| --- | --- | --- |
| [0, 2] | prefix[3] − prefix[0] = 10 − 0 | 10 |
| [1, 3] | prefix[4] − prefix[1] = 12 − 3 | 9 |
| [4, 4] | prefix[5] − prefix[4] = 17 − 12 | 5 |

**Complejidad:** O(n + q) en tiempo y O(n) de espacio extra, con n días y q consultas. Sumar cada rango directamente cuesta O(n·q) en el peor caso.
