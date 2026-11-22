## El error

El rango de búsqueda se inicializa con uno de menos:

```python
lo, hi = 0, len(sorted) - 1
```

El bucle mantiene la respuesta dentro de `[lo, hi]` y devuelve `lo` cuando ambos se encuentran, así que el resultado siempre está entre `0` y `hi`. Las respuestas válidas van de `0` a `len(sorted)` (el caso de "ninguno llega"), y ese último valor es inalcanzable. Con `sorted = [1, 2]` y `x = 3`, el bucle mira el índice 0 (1 < 3, así que `lo = 1`), se detiene porque `lo == hi` y devuelve `1` en lugar de `2`. El Ejemplo 3 devuelve `4` en vez de `5`.

## La corrección

Deja que el rango incluya la posición justo después del final:

```python
lo, hi = 0, len(sorted)
```

`mid` sigue siendo siempre menor que `hi`, así que `sorted[mid]` nunca se sale del arreglo. En TypeScript, inicializa `let hi = sorted.length;`.

**Complejidad:** O(log n) en tiempo y O(1) de espacio extra.
