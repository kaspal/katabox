Ordenar las ventas de una región y tomar la segunda fila falla cuando dos ventas empatan en el máximo. Replantea la segunda: es el mayor importe entre las ventas que están *por debajo* del máximo.
---
Para una región, el máximo es `(SELECT MAX(t.amount_cents) FROM sales t WHERE t.region = r.region)`. La segunda es el `MAX` de los importes `<` ese máximo; si ningún importe queda por debajo, `MAX` sobre ninguna fila es `NULL`, que es justo la respuesta que necesitas.
---
Parte de la lista de regiones, `FROM (SELECT DISTINCT region FROM sales) r`, para que cada región tenga su fila aunque no tenga segunda. Selecciona `r.region` y la subconsulta correlacionada de arriba `AS runner_up_cents`, y termina con `ORDER BY r.region`.
