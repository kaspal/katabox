-- Cleans the names but keeps each direction: Lima-Quito and Quito-Lima both appear.
SELECT DISTINCT lower(trim(origin)) AS city_a, lower(trim(dest)) AS city_b
FROM flights
WHERE lower(trim(origin)) <> lower(trim(dest))
ORDER BY city_a, city_b;
