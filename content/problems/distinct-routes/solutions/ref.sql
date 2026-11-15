SELECT DISTINCT
       min(lower(trim(origin)), lower(trim(dest))) AS city_a,
       max(lower(trim(origin)), lower(trim(dest))) AS city_b
FROM flights
WHERE lower(trim(origin)) <> lower(trim(dest))
ORDER BY city_a, city_b;
