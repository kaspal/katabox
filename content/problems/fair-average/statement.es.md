Un café de juegos de mesa pide a sus clientes que reseñen los juegos a los que jugaron. Cada reseña de `reviews` nombra un juego y le da de 1 a 5 estrellas, pero algunos clientes solo dejan un comentario, y entonces `stars` es `NULL`.

El café quiere una **media justa** para cada juego reseñado: cuántas reseñas dieron estrellas de verdad y la media de esas estrellas. Una reseña sin estrellas no dice nada de la puntuación, así que no debe bajar la media. Un juego cuyas reseñas no tienen ninguna estrella también aparece, con 0 reseñas puntuadas y sin media.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `reviews(id, game, stars)`: `id` es la clave primaria; `game` es el nombre del juego; `stars` va de 1 a 5, o es `NULL` cuando la reseña no dio estrellas.

## Resultado

Devuelve una fila por cada juego con al menos una reseña, con tres columnas:

| columna | valor |
| --- | --- |
| `game` | el nombre del juego |
| `rated` | el número de sus reseñas cuyo `stars` no es `NULL` |
| `avg_stars` | la media de esas estrellas, redondeada a 2 decimales con el `ROUND` de SQLite (un medio se redondea alejándose de cero); `NULL` cuando `rated` es 0 |

Ordena las filas por `game`. Se aceptan valores de `avg_stars` a menos de 0.001 de los esperados.

### Ejemplo 1

`reviews`:

| id | game | stars |
| --- | --- | --- |
| 1 | Hive | 5 |
| 2 | Hive | NULL |
| 3 | Hive | 4 |
| 4 | Dixit | 3 |
| 5 | Dixit | 4 |
| 6 | Dixit | 4 |
| 7 | Azul | NULL |

Resultado:

| game | rated | avg_stars |
| --- | --- | --- |
| Azul | 0 | NULL |
| Dixit | 3 | 3.67 |
| Hive | 2 | 4.5 |

La reseña de Hive sin estrellas se ignora, así que su media es (5 + 4) / 2 = 4.5, no 9 / 3 = 3. Azul tiene una reseña pero ninguna estrella, así que aparece con 0 y `NULL`.

### Ejemplo 2

`reviews`:

| id | game | stars |
| --- | --- | --- |
| 8 | Catan | 2 |

Resultado:

| game | rated | avg_stars |
| --- | --- | --- |
| Catan | 1 | 2.0 |

## Restricciones

- 1 ≤ reseñas ≤ 3000.
- Los ids son distintos, de 1 a 10⁶.
- El nombre de un juego es una palabra con mayúscula inicial, de 3 a 12 letras.
- `stars` es 1, 2, 3, 4, 5 o `NULL`.
