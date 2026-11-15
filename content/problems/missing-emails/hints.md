There are three ways an email can be missing, and they are not alike: one of them is not a string at all. Can a single comparison catch a `NULL`?
---
`email = NULL` is never true, not even when `email` is `NULL`; the test for a missing value is `email IS NULL`. For the other two cases, remove the spaces first: `trim(email)` strips leading and trailing spaces, so both `''` and `'   '` become `''`.
---
`WHERE email IS NULL OR trim(email) = ''` keeps exactly the readers without a usable email. Select `id, name, city` and finish with `ORDER BY city, name, id`.
