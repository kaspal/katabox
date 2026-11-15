Una fila por año significa `GROUP BY` el año. Los cuatro trimestres no son cuatro grupos sino cuatro columnas del mismo grupo: cada columna suma solo algunas de las filas del grupo.
---
`substr(sold_on, 1, 4)` es el año y `substr(sold_on, 6, 2)` el mes, los dos como texto; `CAST(... AS INTEGER)` los convierte en números. Dentro de un agregado, `CASE WHEN <el mes está en el trimestre> THEN amount_cents ELSE 0 END` cuenta una venta solo en su trimestre.
---
Escribe cuatro de esas sumas, una por rango: `BETWEEN 1 AND 3`, `4 AND 6`, `7 AND 9`, `10 AND 12`, llamadas `q1` a `q4`. Selecciona `CAST(substr(sold_on, 1, 4) AS INTEGER) AS year` y termina con `GROUP BY year ORDER BY year`. El `ELSE 0` es lo que da 0 a un trimestre vacío en vez de `NULL`.
