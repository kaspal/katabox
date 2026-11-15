First get rid of repeated sign-ins on the same day. Then number each learner's days 1, 2, 3, … in date order. Inside a run of consecutive days, the date and the number both go up by one at each step. What stays the same?
---
`SELECT DISTINCT username, day FROM logins` leaves one row per learner and day. Number those rows with `ROW_NUMBER() OVER (PARTITION BY username ORDER BY day)`. Then `julianday(day) - ROW_NUMBER() OVER (...)` has the same value on every day of a run and a new value after each gap: it labels the streaks.
---
Use that difference as a group key: `GROUP BY username, grp` with `COUNT(*)` gives the length of each streak, and a second `GROUP BY username` with `MAX(...) AS longest` keeps the longest one. Chain the three steps with `WITH` and finish with `ORDER BY longest DESC, username`.
