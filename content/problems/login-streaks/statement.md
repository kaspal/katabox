A language-learning app records every sign-in in `logins`: who signed in and on which day. Learners like to keep a **streak** going: a run of consecutive calendar days with at least one sign-in on each day.

The app wants each learner's longest streak so far. Several sign-ins on the same day count as one day, and a streak carries on across the end of a month or a year: 31 December followed by 1 January is two days in a row.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel. A `WITH` query, with as many common table expressions as you need, counts as one `SELECT` statement.

## Tables

- `logins(id, username, day)`: `id` is the primary key; `username` is the learner; `day` is the date of the sign-in, written `YYYY-MM-DD`. A learner may have several rows on the same day.

## Result

Return one row per `username` that appears in `logins`, with two columns:

| column | value |
| --- | --- |
| `username` | the learner |
| `longest` | the number of days in that learner's longest streak |

Order the rows by `longest`, largest first, then by `username`.

### Example 1

`logins`:

| id | username | day |
| --- | --- | --- |
| 1 | ana_r | 2026-01-01 |
| 2 | ana_r | 2026-01-02 |
| 3 | ana_r | 2026-01-02 |
| 4 | ana_r | 2026-01-03 |
| 5 | ana_r | 2026-01-05 |
| 6 | bo99 | 2026-01-01 |
| 7 | bo99 | 2026-01-03 |

Result:

| username | longest |
| --- | --- |
| ana_r | 3 |
| bo99 | 1 |

ana_r signs in on 1, 2 (twice) and 3 January, a streak of 3 days, then misses the 4th. bo99 misses 2 January, so each of their streaks is a single day.

### Example 2

`logins`:

| id | username | day |
| --- | --- | --- |
| 8 | cyrus | 2025-12-31 |
| 9 | cyrus | 2026-01-01 |

Result:

| username | longest |
| --- | --- |
| cyrus | 2 |

## Constraints

- 0 ≤ logins ≤ 3000. With no logins at all, the result is empty.
- Ids are distinct, from 1 to 10⁶.
- A username is 3 to 12 characters of `a`-`z`, `0`-`9` and `_`, starting with a letter.
- Every `day` is a valid date of 2025 or 2026.
