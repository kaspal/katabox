## Enfoque: ordenar y recorrer los tramos

Ordena las pinceladas por `start`. Recorriéndolas en ese orden, la valla pintada es una serie de **tramos** disjuntos:

- si una pincelada empieza en el final del tramo actual o antes, pertenece a ese tramo, que puede crecer: `runEnd = max(runEnd, end)`;
- si no, hay un hueco: el tramo actual está completo, así que suma su longitud y empieza uno nuevo con esta pincelada.

Tras la última pincelada, suma el tramo final.

### Paso a paso

`strokes = [[2, 6], [10, 12], [4, 8]]`, ordenadas: `[2, 6], [4, 8], [10, 12]`:

| pincelada | tramo antes | acción | tramo después | total |
| --- | --- | --- | --- | --- |
| [2, 6] | — | empezar | [2, 6) | 0 |
| [4, 8] | [2, 6) | 4 ≤ 6: alargar | [2, 8) | 0 |
| [10, 12] | [2, 8) | hueco: cerrar (6) | [10, 12) | 6 |
| fin | | cerrar (2) | | 8 |

**Complejidad:** O(n log n) en tiempo por la ordenación y O(n) para el recorrido. Fusionar cada pincelada en una lista de tramos recorriendo la lista entera cuesta O(n²) cuando las pinceladas no se solapan.
