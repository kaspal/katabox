## Enfoque: contar las fichas

Dos bolsas son gemelas justo cuando tienen el mismo número de cada letra.

1. Si las longitudes son distintas, devuelve `false` directamente.
2. Usa un arreglo de 26 contadores. Para cada posición, suma 1 por `a[i]` y resta 1 por `b[i]`.
3. Las bolsas son gemelas si todos los contadores valen 0.

### Paso a paso

`a = "tool"`, `b = "lots"`: tras el bucle, `o` queda en +1 (dos en `a`, una en `b`) y `s` en −1, así que la respuesta es `false`.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra (26 contadores). Ordenar las dos cadenas y compararlas también es correcto, en O(n log n).
