The mentor's name is in the same table as the person's name, just in a different row. How can one query read `staff` twice?
---
Give the table two aliases: `staff s` for the person and `staff m` for the mentor, matched with `m.id = s.mentor_id`. A plain `JOIN` keeps only the rows that find a match, though, and someone with no mentor finds none. Which join keeps them?
---
`FROM staff s LEFT JOIN staff m ON m.id = s.mentor_id` keeps everyone and fills the mentor's columns with `NULL` when there is no match. Select `s.name, m.name AS mentor` and finish with `ORDER BY s.name, s.id`.
