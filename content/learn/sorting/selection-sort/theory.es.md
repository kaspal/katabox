## La idea

La ordenación por selección divide el arreglo en una parte ordenada a la izquierda y una parte sin ordenar a la derecha. Cada pasada recorre toda la parte sin ordenar buscando su menor valor y lo intercambia al principio de esa parte, donde se queda para siempre. Tras `n − 1` pasadas el último valor ya está en su sitio, así que el arreglo queda ordenado.

## Pasos

1. Empieza una pasada en la posición `i`: supón que `a[i]` es el menor hasta ahora (`smallest = i`).
2. Compara cada valor posterior `a[j]` con `a[smallest]`.
3. Cuando `a[j]` sea menor, recuerda su posición: `smallest = j`.
4. Al terminar el recorrido, intercambia `a[i]` y `a[smallest]`. La posición `i` ya es definitiva.
5. Mueve `i` un lugar a la derecha y repite hasta que quede un solo valor.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n²)` (el recorrido nunca para antes) |
| Caso medio | `O(n²)` |
| Peor caso | `O(n²)` |
| Espacio extra | `O(1)` |
| Estable | No: el intercambio largo puede saltar por encima de un valor igual |
| En el sitio | Sí |

## Errores comunes

- **Esperar que una entrada ordenada sea rápida.** La ordenación por selección siempre hace `n(n − 1)/2` comparaciones: 45 para los 10 valores por defecto, estén en el orden que estén. Prueba "Casi ordenada" y luego "Invertida": el número de comparaciones no cambia.
- **Intercambiar con cada nuevo mínimo.** Intercambia una sola vez, al final de la pasada. Intercambiar dentro del recorrido hace trabajo de más y lo convierte en una ordenación de burbuja torpe.
- **Suponer que es estable.** En `[5, 5, 2]` la primera pasada intercambia el 2 con el primer 5, que acaba detrás del segundo 5. Se pierde el orden original de los valores iguales.

## Cuándo usarlo

La ordenación por selección hace como mucho `n − 1` intercambios, menos que cualquier otra ordenación sencilla, así que puede tener sentido cuando escribir un valor cuesta mucho más que leerlo. En los demás casos, la ordenación por inserción es igual de corta y se adapta a los datos casi ordenados. La idea de "elige el menor y repite" vuelve en la ordenación por montículo, que encuentra cada siguiente valor con un montículo en lugar de recorrerlo todo.

## Temas relacionados

- [Ordenación por inserción](learn:sorting/insertion-sort): la misma familia `O(n²)`, pero rápida con entradas casi ordenadas.
- [Ordenación de burbuja](learn:sorting/bubble-sort): muchos intercambios por pasada en lugar de uno.
- [Ordenación por montículo](learn:sorting/heap-sort): selección con un montículo, en `O(n log n)`.
