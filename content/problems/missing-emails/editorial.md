## Idea

One table, one filter, one sort. The only real work is the filter, because "missing" covers two different kinds of value:

1. **No value**: the column is `NULL`.
2. **A blank value**: a string that is empty once its spaces are removed.

## The query

```sql
SELECT id, name, city
FROM readers
WHERE email IS NULL OR trim(email) = ''
ORDER BY city, name, id;
```

## Details that matter

- **`= NULL` matches nothing.** Any comparison with `NULL` gives `NULL`, which `WHERE` treats as "not true". So `email = NULL` drops every reader who has no email at all, the very readers the editor most wants to call. The test for a missing value is `IS NULL`.
- **Spaces count as blank.** `email = ''` finds the empty string but not `'   '`. `trim(email)` with one argument removes spaces from both ends, which turns any all-space value into `''`. (It removes only spaces; here no other blank character can appear.)
- **Order of the two tests.** For a `NULL` email, `trim(email) = ''` is itself `NULL`, but `OR` with a true `email IS NULL` is still true, so the order in which you write the two conditions does not change the result.
- **A full order.** Two readers can share a name and a city, so the `id` breaks the last tie.

## Cost

One pass over the readers to test each email, then a sort of the rows that are kept.
