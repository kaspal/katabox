## La idea

La ordenación por mezcla divide el tramo en dos mitades, ordena cada mitad de la misma manera y luego **mezcla** las dos mitades ordenadas en una sola. La división para en los tramos de un solo valor, que ya están ordenados. Todo el trabajo de verdad ocurre en `merge`: copia `a[lo..hi]` a un búfer (`buffer`), recorre las dos mitades con un índice cada una y en cada paso escribe en `a` la menor de las dos cabezas.

## Pasos

1. Si el tramo `lo..hi` tiene un solo valor (`lo >= hi`), termina.
2. Divide en `mid = (lo + hi) // 2`; ordena `lo..mid` y luego `mid + 1..hi`.
3. Copia `a[lo..hi]` al búfer. `i` empieza en la cabeza de la mitad izquierda y `j` en la de la derecha.
4. Para cada posición `k` de `lo` a `hi`, escribe la cabeza menor en `a[k]` y avanza ese índice. Ante un empate, gana la cabeza izquierda.
5. Cuando una mitad se acaba, lo que queda de la otra va detrás, en orden.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n log n)` |
| Caso medio | `O(n log n)` |
| Peor caso | `O(n log n)` (la división nunca depende de los valores) |
| Espacio extra | `O(n)` para el búfer, más `O(log n)` de pila |
| Estable | Sí: ante un empate va primero la cabeza izquierda |
| En el sitio | No |

## Errores comunes

- **Tomar la cabeza derecha en los empates.** Comparar con `<` en lugar de `<=` deja que un valor igual de la mitad derecha adelante a uno de la izquierda, y la ordenación deja de ser estable.
- **Olvidar lo que sobra.** Cuando una mitad se acaba, a la otra todavía le quedan valores. Las comprobaciones `j > hi - lo` (la mitad derecha se acabó) e `i <= mid - lo` (a la izquierda aún le quedan) cubren los dos casos en una sola condición.
- **Mezclar índices.** El búfer empieza en 0, no en `lo`, así que la mitad derecha empieza en `mid + 1 - lo`. Confundir posiciones del búfer y del arreglo es el error clásico de la mezcla.
- Todas las entradas cuestan casi lo mismo: cada uno de los unos `log₂ n` niveles escribe los `n` valores; solo cambia un poco el número de comparaciones.

## Cuándo usarlo

Usa la ordenación por mezcla cuando necesites un `O(n log n)` garantizado o una ordenación estable, por ejemplo para ordenar registros por una clave y luego por otra. También es la opción natural para listas enlazadas, que se pueden mezclar sin búfer, y para datos que no caben en memoria, donde se mezclan trozos ordenados en disco.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): la misma división por la mitad.
- [Tim Sort (simplificado)](learn:sorting/tim-sort): tramos ordenados por inserción y mezclados con este mismo `merge`.
- [Ordenación rápida](learn:sorting/quick-sort): hace su trabajo antes de las llamadas recursivas en lugar de después, y no necesita búfer.
