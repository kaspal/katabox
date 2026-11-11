## Enfoque: ordenar inicios y finales, y recorrerlos

En cada momento, las reuniones en curso necesitan salas distintas, y en cuanto una sala queda libre se puede volver a usar. Así que la respuesta es el máximo número de reuniones en curso a la vez.

Ordena por separado las horas de inicio y las de fin. Recorre ambas listas en orden de tiempo: cuando el siguiente evento es un inicio, hay una sala más en uso; cuando es un final, se libera una. Si un inicio y un final ocurren en el mismo minuto, procesa primero el final (la sala está libre desde `end`). Guarda el máximo.

### Paso a paso

`meetings = [[1, 5], [2, 6], [3, 7], [6, 8]]`: inicios `[1, 2, 3, 6]`, finales `[5, 6, 7, 8]`.

| evento | salas en uso | máximo |
| --- | --- | --- |
| inicio 1 | 1 | 1 |
| inicio 2 | 2 | 2 |
| inicio 3 | 3 | 3 |
| final 5 | 2 | 3 |
| final 6 (antes del inicio 6) | 1 | 3 |
| inicio 6 | 2 | 3 |

El resto son finales; la respuesta es 3.

**Complejidad:** O(n log n) en tiempo por la ordenación y O(n) de espacio extra. Contar, para cada reunión, cuántas otras están en curso cuando empieza cuesta O(n²).
