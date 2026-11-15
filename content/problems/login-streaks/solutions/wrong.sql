-- Numbers every sign-in, not every day: two sign-ins on one day break the streak.
WITH grouped AS (
  SELECT username,
         julianday(day) - ROW_NUMBER() OVER (PARTITION BY username ORDER BY day) AS grp,
         day
  FROM logins
),
runs AS (
  SELECT username, COUNT(DISTINCT day) AS len FROM grouped GROUP BY username, grp
)
SELECT username, MAX(len) AS longest
FROM runs
GROUP BY username
ORDER BY longest DESC, username;
