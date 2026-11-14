## La idea

La búsqueda binaria normal para en cualquier copia del objetivo. A menudo necesitas más: ¿dónde empiezan las copias y dónde terminan? Dos búsquedas muy parecidas lo responden sobre valores ordenados.

- La **cota inferior** es la primera posición cuyo valor es `≥ target`.
- La **cota superior** es la primera posición cuyo valor es `> target`.

Las copias del objetivo quedan justo entre las dos, así que `upper − lower` es cuántas hay. Si el objetivo no está, las dos cotas caen en el mismo sitio: donde habría que insertarlo para mantener el orden. Cualquiera de las dos puede valer `n`, una más allá del final, cuando todos los valores son menores.

Las dos búsquedas trabajan sobre un rango **semiabierto** `[lo, hi)`: `lo` está incluido y `hi` no, y `hi` empieza en `len(a)` para que `n` sea una respuesta posible. En lugar de parar al encontrar el objetivo, siguen estrechando hasta que `lo == hi`, y ese único sitio es la respuesta.

## Pasos

1. Pon `lo = 0` y `hi = len(a)`.
2. Mientras `lo < hi`, toma `mid = (lo + hi) // 2`.
3. Cota inferior: si `a[mid] < target`, la respuesta está a la derecha, así que `lo = mid + 1`; si no, `mid` puede ser la respuesta, así que `hi = mid` (se conserva).
4. Cota superior: lo mismo, pero comparando `a[mid] <= target`.
5. Cuando `lo == hi`, devuelve `lo`.
6. El número de copias es `upper − lower`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(log n)`: no hay salida temprana, cada búsqueda se estrecha hasta un sitio |
| Caso medio | `O(log n)` |
| Peor caso | `O(log n)`, dos veces: una búsqueda por cota |
| Espacio extra | `O(1)` |

## Errores comunes

- **`hi = mid - 1`.** En esta forma `mid` puede ser la respuesta, así que tiene que seguir en el rango: `hi = mid`. Si lo quitas, te saltas justo la posición que buscabas.
- **`hi = len(a) - 1`.** La respuesta puede ser `n` (todos los valores son menores que el objetivo). Empezar `hi` en el último índice hace imposible esa respuesta.
- **Leer `a[lower]` sin comprobar.** Una cota inferior de `n` no es un índice válido, y un índice válido puede contener un valor mayor. El objetivo está solo si `lower < n` y `a[lower] == target`.
- **Confundir `<` y `<=`.** Ese único carácter es toda la diferencia entre las dos cotas.

## Cuándo usarlo

Usa las cotas para contar copias de un valor, encontrar la primera o la última aparición, contar valores dentro de un rango `[x, y]` (`lower(y + 1) − lower(x)` con enteros) o saber dónde insertar un valor para que la lista siga ordenada. `bisect_left` y `bisect_right` de Python y `lower_bound` y `upper_bound` de C++ son exactamente estas dos búsquedas.

## Temas relacionados

- [Búsqueda binaria](learn:searching/binary-search): para en cualquier coincidencia.
- [Búsqueda binaria sobre la respuesta](learn:searching/binary-search-answer): la misma forma de "conservar `mid`", sobre un rango de respuestas.
- [Ordenación por inserción](learn:sorting/insertion-sort): una cota inferior o superior dice dónde va cada clave.
