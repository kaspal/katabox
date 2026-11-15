-- Counts a review without stars as 0 stars, which drags the average down.
SELECT game,
       COUNT(stars) AS rated,
       ROUND(AVG(COALESCE(stars, 0)), 2) AS avg_stars
FROM reviews
GROUP BY game
ORDER BY game;
