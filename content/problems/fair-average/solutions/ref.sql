SELECT game,
       COUNT(stars) AS rated,
       ROUND(AVG(stars), 2) AS avg_stars
FROM reviews
GROUP BY game
ORDER BY game;
