## Enfoque: dos montículos alrededor del centro

Mantén dos montículos:

- `bajo`, un **montículo de máximos** con la mitad menor de las lecturas;
- `alto`, un **montículo de mínimos** con la mitad mayor;

con el invariante de que todo valor de `bajo` es ≤ que todo valor de `alto`, y de que `bajo` tiene el mismo tamaño que `alto` o uno más.

Para cada lectura nueva `x`:

1. Si `bajo` está vacío o `x ≤ max(bajo)`, mete `x` en `bajo`; si no, en `alto`.
2. Reequilibra: si `bajo` tiene dos más que `alto`, pasa `max(bajo)` a `alto`; si `alto` tiene más que `bajo`, pasa `min(alto)` a `bajo`.
3. La mediana es `max(bajo)` cuando la cantidad es impar, y `(max(bajo) + min(alto)) / 2` cuando es par.

Pasar la cima de un montículo al otro mantiene el orden, porque esa cima es el valor más cercano a la otra mitad.

### Paso a paso

`readings = [12, -4, 30, 7]`:

| lectura | bajo | alto | mediana |
| --- | --- | --- | --- |
| 12 | {12} | {} | 12 |
| −4 | {−4} | {12} | (−4 + 12) / 2 = 4 |
| 30 | {−4, 12} | {30} | 12 |
| 7 | {−4, 7} | {12, 30} | (7 + 12) / 2 = 9.5 |

(Con la lectura 30, `bajo` recuperó el 12 de `alto` para equilibrar los tamaños; con la 7, el 12 volvió a subir).

Las medianas son mitades exactas de enteros, así que se escriben como decimales exactos en todos los lenguajes; la tolerancia de 10⁻⁶ es solo un margen de seguridad.

**Complejidad:** O(n log n) de tiempo y O(n) de espacio. Volver a ordenar cada prefijo cuesta O(n² log n), e insertar en una lista ordenada son O(n²) movimientos de elementos.
