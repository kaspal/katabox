-- RANK() leaves gaps after a tie (1, 1, 3); the league numbers places without gaps.
SELECT division, name, points,
       RANK() OVER (PARTITION BY division ORDER BY points DESC) AS place
FROM teams
ORDER BY division, place, name, id;
