## Idea

The contact is a chain of fallbacks, which is what `COALESCE` is for:

1. **The preferred channel's value**, if there is a preference and that value exists.
2. **The phone**, then **the email**.
3. **`'no contact'`**, which is never `NULL`, so the chain always ends with a value.

Once the contact can no longer be `NULL`, building the line is plain concatenation.

## The query

```sql
SELECT id,
       last_name || ', ' || first_name || ': ' ||
       COALESCE(CASE prefers WHEN 'phone' THEN phone WHEN 'email' THEN email END,
                phone, email, 'no contact') AS contact_line
FROM clients
ORDER BY last_name, first_name, id;
```

## Details that matter

- **The preference comes first.** `COALESCE(phone, email, 'no contact')` alone ignores `prefers`: a client who has both and prefers email would be shown the phone. The `CASE` puts the preferred value at the front of the chain.
- **A `CASE` without `ELSE` gives `NULL`.** When `prefers` is `NULL`, no `WHEN` matches (a `NULL` equals nothing), and when the preferred channel is itself `NULL` the `CASE` returns `NULL` too. In both cases `COALESCE` moves on to the phone and then the email, which is the fallback the shop asked for. So a client who prefers the phone but has none gets the email.
- **`||` with `NULL` is `NULL`.** Concatenating a `NULL` phone directly would turn the whole line into `NULL`. Resolving the contact inside `COALESCE` first, with a non-`NULL` last resort, keeps every line a string.
- **Exact separators.** The line is `last_name`, a comma and a space, `first_name`, a colon and a space, then the contact: `Rios, Ana: ana@rios.test`.
- **A full order.** Two clients can share both names, so the `id` breaks the last tie.

## Cost

One pass over the clients to build each line, then a sort by name.
