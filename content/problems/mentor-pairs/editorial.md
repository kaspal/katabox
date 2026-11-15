## Idea

This is a **self-join**: the same table plays two roles. One copy of `staff` (alias `s`) is the person, the other (alias `m`) is the mentor, and `m.id = s.mentor_id` links them. Because some people have no mentor, the join must be a `LEFT JOIN`, which keeps every row of `s` even when no `m` matches.

## The query

```sql
SELECT s.name, m.name AS mentor
FROM staff s
LEFT JOIN staff m ON m.id = s.mentor_id
ORDER BY s.name, s.id;
```

## Details that matter

- **An inner join loses people.** With a plain `JOIN`, a person whose `mentor_id` is `NULL` matches no row (`m.id = NULL` is never true), so Ada in Example 1 and Eli in Example 2 would disappear. `LEFT JOIN` keeps them and gives `m.name` the value `NULL`.
- **Keep extra conditions out of `WHERE`.** A filter on `m` written in `WHERE` (say `WHERE m.name <> 'Ada'`) is false for the `NULL` rows and quietly turns the `LEFT JOIN` back into an inner one. Conditions about the mentor belong in `ON`.
- **Which name is which.** Both copies have a `name` column, so qualify every column. The result's second column must be called `mentor`, and the order is by the person's name (`s.name`), not the mentor's.
- **Names repeat.** Two people can share a name, so `s.id` breaks the tie. Ordering by `name` alone would leave their order to chance.
- **Mutual mentors are fine.** If two people mentor each other, each row still looks up a single mentor, so the join never loops.

## Cost

`m.id` is the primary key, so each person's mentor is found with one index lookup: about one step per staff member, then a sort by name.
