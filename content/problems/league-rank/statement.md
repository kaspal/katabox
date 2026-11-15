A five-a-side league keeps its table in `teams`: each team's name, its division and the points it has so far. At the end of the season every division prints its standings.

Within a division, the team with the most points is in **place 1**. Teams with equal points share a place, and the next place number follows on without gaps: three teams with 30, 30 and 27 points are in places 1, 1 and 2. Divisions are ranked separately.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `teams(id, name, division, points)`: `id` is the primary key; `name` is the team's name; `division` is the division's letter; `points` is the team's points.

## Result

Return one row per team, with four columns:

| column | value |
| --- | --- |
| `division` | the team's division |
| `name` | the team's name |
| `points` | the team's points |
| `place` | the team's place within its division |

Order the rows by `division`, then by `place`, then by `name`, then by the team's `id`.

### Example 1

`teams`:

| id | name | division | points |
| --- | --- | --- | --- |
| 1 | Rovers | A | 30 |
| 2 | United | A | 27 |
| 3 | Albion | A | 30 |
| 4 | Town | A | 25 |
| 5 | City | B | 12 |

Result:

| division | name | points | place |
| --- | --- | --- | --- |
| A | Albion | 30 | 1 |
| A | Rovers | 30 | 1 |
| A | United | 27 | 2 |
| A | Town | 25 | 3 |
| B | City | 12 | 1 |

Albion and Rovers share place 1 in division A and are listed by name; United is next, in place 2, not 3. City is alone in division B, so it is in place 1.

### Example 2

`teams`:

| id | name | division | points |
| --- | --- | --- | --- |
| 6 | Rangers | C | 0 |

Result:

| division | name | points | place |
| --- | --- | --- | --- |
| C | Rangers | 0 | 1 |

## Constraints

- 1 ≤ teams ≤ 2000.
- Ids are distinct, from 1 to 10⁶.
- A name is one capitalised word of 3 to 12 letters; two teams may have the same name.
- `division` is `A`, `B`, `C` or `D`.
- 0 ≤ `points` ≤ 300.
