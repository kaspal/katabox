## La idea

Un **montículo de máximos** es un arreglo leído como árbol binario: los hijos de la posición `i` son `2i + 1` y `2i + 2`, y cada padre es al menos tan grande como sus hijos, así que el mayor valor está en la raíz, `a[0]`. La ordenación por montículo primero convierte el arreglo en un montículo de máximos. Después intercambia la raíz al final, reduce el montículo en uno y lo repara. Cada ronda pone el siguiente mayor valor en su lugar definitivo, llenando el arreglo desde la derecha.

## Pasos

1. Construye el montículo: para cada padre, desde el último (`n // 2 - 1`) hasta la raíz, **húndelo**.
2. Para hundir, elige el hijo mayor. Si el padre es menor, intercámbialos y sigue desde el sitio del hijo; si no, para.
3. Intercambia la raíz (el mayor valor) con el último valor del montículo, `a[end]`. La posición `end` ya es definitiva.
4. Ahora el montículo termina justo antes de `end`. Hunde la nueva raíz para repararlo.
5. Repite hasta que el montículo tenga un solo valor.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n log n)` |
| Caso medio | `O(n log n)` |
| Peor caso | `O(n log n)` |
| Espacio extra | `O(1)` (hundir es un bucle, no una recursión) |
| Estable | No: la raíz salta al final, por encima de valores iguales |
| En el sitio | Sí |

## Errores comunes

- **Hijos más allá del final.** Un padre puede tener un hijo o ninguno. Comprueba `child < size` antes de leer un hijo, y `child + 1 < size` antes de comparar los dos.
- **Hundir con el tamaño equivocado.** En la segunda fase el montículo es solo `a[0..end-1]`. Hundir con `n` volvería a meter en el montículo los valores grandes ya colocados al final.
- **Construir desde la raíz hacia abajo.** Hundir solo funciona cuando los dos subárboles ya son montículos, así que la construcción va del último padre hasta la raíz. Hecha así, construir cuesta solo `O(n)`.
- **Lenta en la práctica.** En arreglos grandes, padres e hijos quedan lejos en memoria, lo que castiga a la caché, así que la ordenación por montículo suele ser más lenta que la rápida pese a tener el mismo `O(n log n)`.

## Cuándo usarlo

La ordenación por montículo da un `O(n log n)` garantizado sin memoria extra; por eso las ordenaciones de biblioteca recurren a ella cuando la recursión de la ordenación rápida se hace demasiado profunda. El montículo en sí suele ser más útil que la ordenación: da el mayor valor en `O(1)` y admite un valor nuevo en `O(log n)`, justo lo que necesita una cola de prioridad.

## Temas relacionados

- [Ordenación por selección](learn:sorting/selection-sort): la misma idea de "saca el valor extremo", con un recorrido lento.
- [Ordenación rápida](learn:sorting/quick-sort): normalmente más rápida, pero sin la garantía.
- Montículo binario y cola de prioridad (próximamente).
