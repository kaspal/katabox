## Enfoque: una criba y una cuenta acumulada

Sea `M` el mayor límite. La **criba de Eratóstenes** marca de una vez los primos hasta `M`:

```
primo = [cierto] * (M + 1); primo[0] = primo[1] = falso
para p = 2 mientras p * p <= M:
    si primo[p]:
        para m = p * p, p * p + p, ... <= M: primo[m] = falso
```

Todo compuesto `m` tiene un factor primo `p ≤ √m`, así que ese `p` lo tacha, y empezar en `p²` no se deja ninguno. Después construye la cuenta acumulada `cuenta[x] = cuenta[x − 1] + (1 si primo[x])` y responde cada pregunta con `cuenta[L]`.

### Paso a paso

Hasta 30 la criba tacha los múltiplos de 2 (desde 4), de 3 (desde 9) y de 5 (desde 25). Quedan 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, así que `cuenta[30] = 10` y `cuenta[10] = 4`.

**Complejidad:** O(M log log M) para la criba, O(M) para la cuenta y O(1) por pregunta, con O(M) de memoria. Probar divisores para cada número hasta `L`, en cada pregunta, cuesta unas `q · L · √L` operaciones.
