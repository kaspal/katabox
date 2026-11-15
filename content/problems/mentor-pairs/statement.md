A newsroom pairs its reporters with mentors. Everyone is listed in a single table, `staff`, and each row says who mentors that person, if anyone. A mentor is just another member of the staff.

The editor wants the full **mentor list**: every staff member next to the name of their mentor. People without a mentor must still be on the list, with an empty mentor.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `staff(id, name, mentor_id)`: `id` is the primary key; `mentor_id` is the `id` of this person's mentor in the same table, or `NULL` when they have none. The schema declares no `REFERENCES`, but every non-`NULL` `mentor_id` is the id of another staff member.

## Result

Return one row per staff member, with two columns:

| column | value |
| --- | --- |
| `name` | the staff member's name |
| `mentor` | the name of their mentor, or `NULL` when they have none |

Order the rows by the staff member's `name`, then by their `id`.

### Example 1

`staff`:

| id | name | mentor_id |
| --- | --- | --- |
| 1 | Ada | NULL |
| 2 | Bea | 1 |
| 3 | Carl | 1 |
| 4 | Dina | 3 |

Result:

| name | mentor |
| --- | --- |
| Ada | NULL |
| Bea | Ada |
| Carl | Ada |
| Dina | Carl |

Ada has no mentor, so her row has `NULL`. Ada mentors both Bea and Carl, and Carl in turn mentors Dina.

### Example 2

`staff`:

| id | name | mentor_id |
| --- | --- | --- |
| 5 | Eli | NULL |

Result:

| name | mentor |
| --- | --- |
| Eli | NULL |

## Constraints

- 1 ≤ staff ≤ 500; the ids are distinct, from 1 to 10⁶.
- A name is one capitalised word. Names may repeat: two staff members can share a name, and then the one with the smaller `id` comes first.
- `mentor_id` is `NULL` or the id of a different staff member. One person can mentor several others, and two people can mentor each other.
