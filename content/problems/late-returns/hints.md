The names live in one table and the loans in another. How do you put each loan next to the member who took it?
---
`JOIN members m ON m.id = l.member_id` gives every loan its member. Dates written `YYYY-MM-DD` compare correctly as text, so `l.returned > l.due` keeps exactly the late ones; a `NULL` return date never passes that test.
---
SQLite's `julianday(d)` turns a date into a day number, so `julianday(l.returned) - julianday(l.due)` is the number of days late; wrap it in `CAST(... AS INTEGER)`. Name the column `days_late` and finish with `ORDER BY days_late DESC, l.title, l.id`.
