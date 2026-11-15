A small newsletter keeps its readers in one table. When someone signs up at a fair, the volunteer at the stand types whatever they get: sometimes a proper address, sometimes nothing at all, and sometimes just a few taps on the space bar.

Before the next issue goes out, the editor wants to call everyone who has **no usable email**. An email is missing when it is `NULL`, when it is the empty string, or when it is made only of spaces.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `readers(id, name, email, city)`: `id` is the primary key; `name` is the reader's name; `email` is what was typed for the address, or `NULL` when nothing was recorded; `city` is where the reader lives.

## Result

Return one row per reader whose email is missing, with three columns:

| column | value |
| --- | --- |
| `id` | the reader's `id` |
| `name` | the reader's name |
| `city` | the reader's city |

Order the rows by `city`, then by `name`, then by `id`.

### Example 1

`readers`:

| id | name | email | city |
| --- | --- | --- | --- |
| 1 | ana | ana@mail.test | Lima |
| 2 | bo | NULL | Quito |
| 3 | cy | `''` (empty) | Lima |
| 4 | di | `'   '` (three spaces) | Bogota |
| 5 | ed | ed@mail.test | Quito |

Result:

| id | name | city |
| --- | --- | --- |
| 4 | di | Bogota |
| 3 | cy | Lima |
| 2 | bo | Quito |

Reader bo has no email at all, cy's is the empty string and di's is three spaces, so all three are missing; they come out by city: Bogota, Lima, Quito.

### Example 2

`readers`:

| id | name | email | city |
| --- | --- | --- | --- |
| 7 | fay | fay@mail.test | Cusco |

Result: no rows (the columns are still `id`, `name`, `city`).

## Constraints

- 1 ≤ readers ≤ 300; the ids are distinct, from 1 to 10⁶.
- A name is 1 to 12 lowercase letters; a city is one capitalised word (`Lima`, `Quito`).
- An email is `NULL` or up to 30 characters, each a lowercase letter, `@`, `.` or a space. Spaces are the only blank characters that appear.
- Two readers may share a name and a city.
