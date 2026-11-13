## Enfoque: ordenación por mezcla con una pasada de recuento

Toda caída brusca `(i, j)` tiene los dos días en la primera mitad, los dos en la segunda, o `i` en la primera y `j` en la segunda. Los dos primeros tipos se cuentan de forma recursiva. Para el tercero, el orden de los días **dentro** de cada mitad ya no importa, solo que el día de la primera mitad es anterior, así que las dos mitades se pueden ordenar.

Con `left` y `right` ordenadas de menor a mayor, avanza un puntero `j` por `right` mientras `x > 2 · right[j]`, para cada `x` de `left` en orden. Como `x` solo crece, `j` nunca retrocede, y `j` es el número de caídas bruscas que empiezan en `x` y terminan en la mitad derecha. Eso es O(longitud) por paso de mezcla.

Después mezcla `left` y `right` en una lista ordenada para quien llamó.

Fíjate en la diferencia con contar inversiones: allí el recuento puede hacerse durante la propia mezcla, pero aquí la condición `x > 2y` no coincide con el `x > y` de la mezcla, así que el recuento necesita su propia pasada.

### Paso a paso

`[9, 4, 3, 10, 2]` se divide en `[9, 4]` y `[3, 10, 2]`. Dentro: (9, 4) es una caída; en la parte derecha, (10, 2). Entre las mitades, con `[4, 9]` y `[2, 3, 10]` ordenadas: 4 no es > 2·2, así que 0; 9 > 4 y 9 > 6 pero no > 20, así que 2. Total 1 + 1 + 2 = 4.

**Complejidad:** O(n log n) en tiempo y O(n) de espacio extra. Comprobar cada par es O(n²), 2·10¹⁰ comprobaciones para 2·10⁵ días.
