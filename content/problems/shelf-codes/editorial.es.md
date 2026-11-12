## Enfoque: búsqueda binaria del primer número ≥ x

Como `codes` está ordenada, los números menores que `x` son exactamente los primeros. Así que la respuesta es el índice del primer elemento que **no** es menor que `x`: la "cota inferior" de `x`.

```
lo, hi = 0, len(codes)          # la respuesta está en [lo, hi]
mientras lo < hi:
    mid = (lo + hi) // 2
    si codes[mid] < x: lo = mid + 1   # mid y todo lo anterior son menores
    si no: hi = mid                   # el primer elemento ≥ x está en mid o antes
devolver lo
```

Cada vuelta divide el rango a la mitad, así que una consulta cuesta unas log₂(10⁵) ≈ 17 comparaciones. Los números repetidos no necesitan nada especial: el bucle pasa todas las copias menores que `x` y se detiene en la primera copia igual.

### Paso a paso

`codes = [3, 8, 8, 15, 20]`, `x = 16`: el rango [0, 5] se estrecha con mid 2 (8 < 16, lo = 3), mid 4 (20 ≥ 16, hi = 4), mid 3 (15 < 16, lo = 4). Respuesta 4.

**Complejidad:** O(q log n) de tiempo y O(1) de espacio extra por consulta. Contar recorriendo toda la lista en cada consulta cuesta O(n · q).
