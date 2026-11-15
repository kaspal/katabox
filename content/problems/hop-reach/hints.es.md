Piensa por rondas: primero los muelles a una travesía de Central, luego los que están a una travesía de esos, y así sucesivamente. La red puede volver sobre sí misma, así que ¿qué hace que las rondas se detengan?
---
`WITH RECURSIVE reach(pier, hops)` empieza con `SELECT 'Central', 0`. La parte recursiva sigue una ruta más: `JOIN routes r ON r.from_pier = reach.pier`, produciendo `r.to_pier, reach.hops + 1`. Una condición `WHERE reach.hops < 3` solo extiende las filas a las que aún les queda una travesía, lo que además termina el recorrido en cualquier ciclo.
---
A un muelle se puede llegar por varios caminos de distinta longitud, así que termina con `SELECT pier, MIN(hops) AS hops FROM reach WHERE pier <> 'Central' GROUP BY pier ORDER BY hops, pier`. Unir las dos partes con `UNION` en lugar de `UNION ALL` descarta las filas `(pier, hops)` repetidas y mantiene el trabajo pequeño.
