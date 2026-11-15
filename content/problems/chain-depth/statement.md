A hospital keeps its whole staff in one table, `staff`. Everyone except the director reports to exactly one person, whose `id` is in `boss_id`; the director's `boss_id` is `NULL`.

For a new organisation chart, the hospital wants to know how far down the reporting line each person sits. Their **depth** is the number of people above them: their boss, their boss's boss and so on, up to and including the director. The director has depth 0, the people who report to the director have depth 1, and so on.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel. A `WITH RECURSIVE` query counts as one `SELECT` statement.

## Tables

- `staff(id, name, boss_id)`: `id` is the primary key; `boss_id` is the `id` of the person this one reports to, or `NULL` for the director.

Exactly one person, the director, has `boss_id` `NULL`. The schema declares no foreign key, but every other `boss_id` is the `id` of another staff member, and there are no loops: following the bosses upward from anyone always ends at the director. A reporting line can be up to 1000 people long.

## Result

Return one row per staff member, with two columns:

| column | value |
| --- | --- |
| `name` | the person's name |
| `depth` | how many people are above them in the reporting line |

Order the rows by `depth`, smallest first, then by `name`, then by the person's `id`.

### Example 1

`staff`:

| id | name | boss_id |
| --- | --- | --- |
| 1 | Iris | NULL |
| 2 | Bram | 1 |
| 3 | Cleo | 1 |
| 4 | Dev | 3 |
| 5 | Ada | 4 |

Result:

| name | depth |
| --- | --- |
| Iris | 0 |
| Bram | 1 |
| Cleo | 1 |
| Dev | 2 |
| Ada | 3 |

Iris is the director. Bram and Cleo report to her, Dev reports to Cleo and Ada to Dev, so Ada has three people above her: Dev, Cleo and Iris.

### Example 2

`staff`:

| id | name | boss_id |
| --- | --- | --- |
| 9 | Eva | NULL |

Result:

| name | depth |
| --- | --- |
| Eva | 0 |

## Constraints

- 1 ≤ staff ≤ 1000.
- Ids are distinct, from 1 to 10⁶.
- A name is one capitalised word of 2 to 12 letters; two people may share a name.
- Exactly one director, and the reporting lines have no loops.
