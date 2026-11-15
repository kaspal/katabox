Necesitas una fila por día, pero ninguna columna guarda solo el día: `placed` también lleva la hora. ¿Cómo le quitas la hora?
---
`substr(placed, 1, 10)` se queda solo con la parte `AAAA-MM-DD`. Agrupar por ese valor junta todos los pedidos de un día en el mismo grupo, sea cual sea su hora.
---
`SELECT substr(placed, 1, 10) AS day, COUNT(*) AS orders, SUM(total_cents) AS revenue_cents FROM orders GROUP BY day ORDER BY day`. Agrupa por el día, no por `placed`, o cada minuto distinto se convierte en su propia fila.
