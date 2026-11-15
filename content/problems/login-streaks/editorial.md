## Idea

This is the classic **gaps and islands** pattern: the days a learner signed in form "islands" of consecutive days separated by gaps, and we want the size of the biggest island.

The trick: number each learner's distinct days 1, 2, 3, … in date order. Inside an island, each next day is one calendar day later *and* one number higher, so `day − row number` stays constant. A gap moves the day forward by more than one while the number still goes up by one, so the difference jumps to a new value. That difference is a label for the island.

1. **One row per learner and day**: drop repeated sign-ins.
2. **Label** each day with `julianday(day) − ROW_NUMBER()`.
3. **Count** the days of each (learner, label) group, then **keep the largest** per learner.

For ana_r in Example 1, the days 1, 2, 3 and 5 January get the numbers 1, 2, 3 and 4. The first three share one label; 5 January gets a label one higher, so it is a separate island.

## The query

```sql
WITH days AS (
  SELECT DISTINCT username, day FROM logins
),
grouped AS (
  SELECT username,
         julianday(day) - ROW_NUMBER() OVER (PARTITION BY username ORDER BY day) AS grp
  FROM days
),
runs AS (
  SELECT username, COUNT(*) AS len FROM grouped GROUP BY username, grp
)
SELECT username, MAX(len) AS longest
FROM runs
GROUP BY username
ORDER BY longest DESC, username;
```

## Details that matter

- **Distinct days first.** Numbering every sign-in instead of every day breaks the trick: a second sign-in on the same day takes a row number without moving the date, so the difference drops by one and the island splits in two. A learner who signs in on 1, 2 (twice) and 3 January gets the labels `x`, `x`, `x − 1`, `x − 1`: two pieces of 2 days, and an answer of 2 instead of 3. Later days can even land in an earlier piece by accident. Counting `COUNT(DISTINCT day)` per group afterwards does not repair it, because the split has already happened. Remove the duplicates *before* numbering.
- **Calendar days, not text.** `julianday` turns a date into a day number that runs straight across month and year ends, so 28 February to 1 March and 31 December to 1 January are one day apart. Taking the day of the month with `strftime('%d', ...)` or comparing pieces of the text would break at those boundaries.
- **Exact labels.** `julianday` returns a number ending in `.5` for a plain date, and subtracting a whole row number keeps it exact, so grouping by the result is safe.
- **`PARTITION BY username`.** Each learner needs their own numbering from 1; one numbering over everybody would mix their days.
- **Everybody gets at least 1.** Any learner with a sign-in has an island of at least one day, so every username in `logins` appears in the result.

## Cost

Removing duplicates, numbering per learner and the two groupings each sort or hash the rows once: about `O(n log n)` for `n` sign-ins.
