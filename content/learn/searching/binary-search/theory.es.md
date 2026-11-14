## La idea

Cuando los valores están ordenados, una comparación te dice mucho más que "sí" o "no". Mira el valor del medio: si es menor que el objetivo, el objetivo no puede estar a su izquierda, porque ahí todo es aún menor. Así se descarta la mitad del tramo de golpe. La búsqueda binaria mantiene un tramo `lo..hi` que tiene que contener el objetivo si es que está, comprueba su posición central `mid` y tira la mitad que no puede contenerlo. Cada comprobación parte el tramo por la mitad, así que un millón de valores necesitan unas 20 comprobaciones.

El ejemplo devuelve la posición donde encuentra el objetivo, o `-1` cuando el tramo se queda vacío. Con duplicados, es **alguna** posición que coincide, no necesariamente la primera; [Cota inferior y superior](learn:searching/lower-upper-bound) encuentra la primera.

## Pasos

1. Pon `lo = 0` y `hi = len(a) - 1`: todo el arreglo.
2. Mientras `lo <= hi`, toma `mid = (lo + hi) // 2`.
3. Si `a[mid] == target`, devuelve `mid`.
4. Si `a[mid] < target`, el objetivo está a la derecha: `lo = mid + 1`.
5. Si no, está a la izquierda: `hi = mid - 1`.
6. Cuando `lo > hi`, el tramo está vacío: devuelve `-1`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(1)` (el primer centro es el objetivo) |
| Caso medio | `O(log n)` |
| Peor caso | `O(log n)`: unas `log₂ n + 1` comprobaciones, y el tramo queda vacío |
| Espacio extra | `O(1)`: tres índices |

## Errores comunes

- **Entrada sin ordenar.** Con valores desordenados la respuesta sale mal sin ningún aviso. Ordena primero, o comprueba el orden si los datos vienen de fuera.
- **Mover un extremo a `mid` en vez de pasarlo.** Con `lo <= hi`, escribir `lo = mid` puede repetir el mismo `mid` para siempre. `mid` ya está comprobado, así que sáltalo.
- **Desbordamiento al calcular el centro.** En lenguajes con enteros de tamaño fijo, `lo + hi` puede desbordarse en arreglos enormes; `lo + (hi - lo) / 2` lo evita. En Python y JavaScript no pasa.
- **Esperar la primera copia.** En `[2, 5, 5, 5, 9]` con objetivo 5, el primer centro ya cae en un 5 y devuelve 2, no 1.

## Cuándo usarlo

Úsala para consultas en datos ordenados que no cambian a menudo: una lista ordenada de identificadores, las marcas de tiempo de un registro, una tabla de umbrales. Su alcance es mayor: siempre que una pregunta de sí o no cambie una sola vez a lo largo de un rango ordenado, la misma división encuentra el punto del cambio, que es lo que hace la [Búsqueda binaria sobre la respuesta](learn:searching/binary-search-answer).

## Temas relacionados

- [Búsqueda lineal](learn:searching/linear-search): sin necesidad de orden, `O(n)` comprobaciones.
- [Cota inferior y superior](learn:searching/lower-upper-bound): el primer y el último sitio de un valor.
- [Búsqueda binaria sobre la respuesta](learn:searching/binary-search-answer): dividir un rango de respuestas posibles.
- [Búsqueda exponencial](learn:searching/exponential-search): búsqueda binaria cuando el final está lejos o no se conoce.
- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): la búsqueda binaria también se puede escribir de forma recursiva.
