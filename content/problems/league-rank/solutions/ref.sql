SELECT division, name, points,
       DENSE_RANK() OVER (PARTITION BY division ORDER BY points DESC) AS place
FROM teams
ORDER BY division, place, name, id;
