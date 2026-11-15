Cada fila del resultado necesita un valor de otra fila: la fila de la misma empresa justo anterior en el tiempo. ¿Qué tipo de función puede leer una fila vecina sin unir la tabla consigo misma?
---
`LAG(close_cents) OVER (PARTITION BY ticker ORDER BY day)` da el cierre anterior del mismo ticker, o `NULL` en su primer día. Una función de ventana no se puede usar en `WHERE`, así que calcúlala en una subconsulta (o en un `WITH`) y filtra fuera con `prev_cents IS NOT NULL`.
---
En la consulta exterior, `ROUND(100.0 * (close_cents - prev_cents) / prev_cents, 2) AS change_pct`; el `100.0` hace que la división sea real y no entera. Selecciona `ticker, day, close_cents, prev_cents` antes y termina con `ORDER BY ticker, day`.
