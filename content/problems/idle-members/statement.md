A climbing gym keeps its members in `members` and logs every visit in `visits`, one row per member per check-in.

Before renewing memberships, the manager wants to reach out to the **idle members**: everyone who did not come to the gym even once in November 2026. Visits in October, December or any other month do not change that, and a member who has never visited at all is idle too.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `members(id, name)`: `id` is the primary key.
- `visits(id, member_id, day)`: `id` is the primary key; `member_id` refers to `members(id)`; `day` is the date of the visit, written `YYYY-MM-DD`.

## Result

Return one row per member with no visit from `2026-11-01` to `2026-11-30` (both days included), with two columns:

| column | value |
| --- | --- |
| `id` | the member's `id` |
| `name` | the member's name |

Order the rows by `name`, then by `id`.

### Example 1

`members`:

| id | name |
| --- | --- |
| 1 | ana |
| 2 | bo |
| 3 | cy |

`visits`:

| id | member_id | day |
| --- | --- | --- |
| 1 | 1 | 2026-11-03 |
| 2 | 2 | 2026-10-30 |
| 3 | 1 | 2026-11-20 |

Result:

| id | name |
| --- | --- |
| 2 | bo |
| 3 | cy |

Ana came twice in November. Bo's only visit was in October, and Cy never came, so both are idle.

### Example 2

`members`:

| id | name |
| --- | --- |
| 4 | di |

`visits`:

| id | member_id | day |
| --- | --- | --- |
| 5 | 4 | 2026-11-30 |

Result: no rows (only the columns `id` and `name`). November 30 is still November.

## Constraints

- 1 ≤ members ≤ 300, 0 ≤ visits ≤ 3000.
- Member ids are distinct, and so are visit ids, each from 1 to 10⁶.
- A name is 1 to 10 lowercase letters; two members may share a name.
- Every visit belongs to a member, and every `day` is a valid date of 2026. A member may have any number of visits.
