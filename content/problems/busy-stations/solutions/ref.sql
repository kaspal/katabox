SELECT station, COUNT(*) AS trips
FROM (
  SELECT id, start_station AS station FROM trips
  UNION
  SELECT id, end_station FROM trips
)
GROUP BY station
HAVING COUNT(*) >= 3
ORDER BY trips DESC, station;
