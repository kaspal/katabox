SELECT id, title, rating
FROM listings
WHERE hidden = 0
ORDER BY rating DESC, title, id
LIMIT 10 OFFSET 20;
