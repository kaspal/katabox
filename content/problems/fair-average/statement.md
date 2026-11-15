A board-game café asks its guests to review the games they played. Each review in `reviews` names a game and gives it 1 to 5 stars, but some guests only leave a comment, and then `stars` is `NULL`.

The café wants a **fair average** for every reviewed game: how many reviews actually gave stars, and the average of those stars. A review without stars says nothing about the score, so it must not pull the average down. A game whose reviews all lack stars is still listed, with 0 rated reviews and no average.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `reviews(id, game, stars)`: `id` is the primary key; `game` is the game's name; `stars` is 1 to 5, or `NULL` when the review gave no stars.

## Result

Return one row per game that has at least one review, with three columns:

| column | value |
| --- | --- |
| `game` | the game's name |
| `rated` | the number of its reviews whose `stars` is not `NULL` |
| `avg_stars` | the average of those stars, rounded to 2 decimals with SQLite's `ROUND` (a half rounds away from zero); `NULL` when `rated` is 0 |

Order the rows by `game`. Values of `avg_stars` within 0.001 of the expected ones are accepted.

### Example 1

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

Result:

| game | rated | avg_stars |
| --- | --- | --- |
| Azul | 0 | NULL |
| Dixit | 3 | 3.67 |
| Hive | 2 | 4.5 |

Hive's review without stars is ignored, so its average is (5 + 4) / 2 = 4.5, not 9 / 3 = 3. Azul has a review but no stars, so it is listed with 0 and `NULL`.

### Example 2

`reviews`:

| id | game | stars |
| --- | --- | --- |
| 8 | Catan | 2 |

Result:

| game | rated | avg_stars |
| --- | --- | --- |
| Catan | 1 | 2.0 |

## Constraints

- 1 ≤ reviews ≤ 3000.
- Ids are distinct, from 1 to 10⁶.
- A game's name is one capitalised word of 3 to 12 letters.
- `stars` is 1, 2, 3, 4, 5 or `NULL`.
