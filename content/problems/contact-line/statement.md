A repair shop phones or writes to its clients when a repair is ready. Each client may have a phone number, an email address, both or neither, and may have said which channel they prefer.

The front desk wants a printed list with one line per client, written as `last_name, first_name: contact`. The contact is chosen like this:

1. If the client prefers a channel (`prefers` is `'phone'` or `'email'`) and that channel's value is not `NULL`, use it.
2. Otherwise use the phone if it is not `NULL`, and failing that the email. So with no preference, or when the preferred channel is `NULL`, the phone comes first.
3. If both the phone and the email are `NULL`, the contact is the text `no contact`.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `clients(id, first_name, last_name, phone, email, prefers)`: `id` is the primary key; `phone` and `email` are the client's phone number and email address, each `NULL` when unknown; `prefers` is `'phone'`, `'email'`, or `NULL` when the client has no preference.

## Result

Return one row per client, with two columns:

| column | value |
| --- | --- |
| `id` | the client's `id` |
| `contact_line` | `last_name`, then `, `, then `first_name`, then `: `, then the contact |

Order the rows by `last_name`, then by `first_name`, then by `id`.

### Example 1

`clients`:

| id | first_name | last_name | phone | email | prefers |
| --- | --- | --- | --- | --- | --- |
| 1 | Ana | Rios | 555-0101 | ana@rios.test | email |
| 2 | Bruno | Soto | 555-0102 | NULL | email |
| 3 | Clara | Vega | NULL | clara@vega.test | NULL |
| 4 | Dario | Mora | NULL | NULL | phone |

Result:

| id | contact_line |
| --- | --- |
| 4 | Mora, Dario: no contact |
| 1 | Rios, Ana: ana@rios.test |
| 2 | Soto, Bruno: 555-0102 |
| 3 | Vega, Clara: clara@vega.test |

Ana prefers email, so her email wins over her phone. Bruno prefers email but has none, so his phone is used; Clara has no preference and no phone, so her email is used; Dario has neither.

### Example 2

`clients`:

| id | first_name | last_name | phone | email | prefers |
| --- | --- | --- | --- | --- | --- |
| 8 | Elena | Paz | 555-0199 | elena@paz.test | NULL |

Result:

| id | contact_line |
| --- | --- |
| 8 | Paz, Elena: 555-0199 |

## Constraints

- 1 ≤ clients ≤ 300; the ids are distinct, from 1 to 10⁶.
- `first_name` and `last_name` are each one capitalised word (`Ana`, `Rios`); several clients may share both.
- A phone is `NULL` or written `ddd-dddd` (`555-0101`); an email is `NULL` or `name@domain.test` in lowercase letters.
- `prefers` is `'phone'`, `'email'` or `NULL`.
