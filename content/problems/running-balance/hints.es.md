Un total acumulado necesita que cada fila vea las filas anteriores sin dejar de aparecer en el resultado. `GROUP BY` las juntaría; una función de ventana no.
---
`SUM(amount_cents) OVER (PARTITION BY account ORDER BY ...)` reinicia el total en cada cuenta y suma las filas en el orden de la ventana. Ese orden tiene que ser completo: `day` solo deja empatados los movimientos del mismo día, así que añade `id` detrás.
---
Escribe el marco para que cada fila se detenga en sí misma: `SUM(amount_cents) OVER (PARTITION BY account ORDER BY day, id ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS balance_cents`. Selecciona `account, day, amount_cents` con él y termina con `ORDER BY account, day, id`.
