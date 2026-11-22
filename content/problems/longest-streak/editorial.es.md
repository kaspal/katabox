## El error

El código solo anota una racha cuando termina porque el siguiente valor es distinto:

```python
        else:
            best = max(best, run)
            run = 1
    return best
```

A la última racha del arreglo nunca le sigue un valor distinto, así que nunca se compara con `best`. Siempre que la racha más larga está al final, la respuesta se queda corta. Con `values = [1, 2, 2]` el código devuelve `1` en lugar de `2`, y con `values = [5]` el bucle ni siquiera se ejecuta y devuelve `0` en vez de `1`. El Ejemplo 1 devuelve `2` en lugar de `3`.

## La corrección

Ten en cuenta la racha final al salir del bucle:

```python
    return max(best, run)
```

En TypeScript, la última línea pasa a ser `return Math.max(best, run);`.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra.
