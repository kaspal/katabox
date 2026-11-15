Split the work in two: first work out the contact as a single value, then glue the names and the contact into one string. What happens if you glue a `NULL` into a string?
---
`'a' || NULL` is `NULL`, so every piece you concatenate must already be non-`NULL`. `COALESCE(x, y, z)` returns its first argument that is not `NULL`, which is exactly "this, else that, else the other". The preferred value is `CASE prefers WHEN 'phone' THEN phone WHEN 'email' THEN email END`, which is `NULL` when there is no preference.
---
Put that `CASE` first in a `COALESCE`, followed by `phone`, `email` and `'no contact'`. Concatenate `last_name || ', ' || first_name || ': ' || COALESCE(...)`, name it `contact_line`, select `id` with it, and finish with `ORDER BY last_name, first_name, id`.
