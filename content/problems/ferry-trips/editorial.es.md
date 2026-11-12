## Enfoque: emparejar con voracidad y dos punteros

Ordena los pesos. Piensa en el excursionista más pesado `H` y el más ligero `L`:

- Si `L + H > limit`, nadie puede ir con `H`, así que `H` cruza solo.
- Si no, `H` y `L` comparten viaje. Esto nunca empeora: en cualquier plan donde `L` viaje con otro `X`, intercambiar a `L` con el acompañante de `H` mantiene todos los viajes dentro del límite (porque `X ≤ H`).

Los dos casos retiran a `H`, así que cada paso es un viaje:

```
ordenar weights; lo = 0; hi = n - 1; viajes = 0
mientras lo <= hi:
    si weights[lo] + weights[hi] <= limit: lo += 1
    hi -= 1; viajes += 1
```

### Paso a paso

`weights = [60, 45, 80, 35]` ordenado queda `[35, 45, 60, 80]`, `limit = 100`:

| lo | hi | ¿cabe la pareja? | viaje |
| --- | --- | --- | --- |
| 35 | 80 | 115 > 100: no | 80 solo |
| 35 | 60 | 95: sí | 60 + 35 |
| 45 | 45 | (mismo excursionista) | 45 solo |

3 viajes.

**Complejidad:** O(n log n) por la ordenación y después O(n). Buscar entre todos los que quedan el mejor acompañante de cada uno cuesta O(n²).
