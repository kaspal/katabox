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
