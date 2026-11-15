El total de cada categoría es un simple `GROUP BY category` con `SUM(amount_cents)`. Pero la cuota también necesita el total de toda la tabla, que ningún grupo conoce por sí solo. ¿Cómo puede llegar a él una consulta agrupada?
---
Una subconsulta escalar, `(SELECT SUM(amount_cents) FROM sales)`, da el total general como un único valor que puedes usar en cualquier fila. Luego cuidado con la división: en SQLite, un entero dividido entre un entero es un entero, sin los decimales.
---
Empieza la expresión con un número real para que la división conserve los decimales: `ROUND(100.0 * SUM(amount_cents) / (SELECT SUM(amount_cents) FROM sales), 1) AS share_pct`. Termina con `ORDER BY share_pct DESC, category`.
