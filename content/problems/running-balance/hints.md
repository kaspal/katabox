A running total needs every row to see the rows before it, while still keeping each row in the result. `GROUP BY` would collapse them; a window function does not.
---
`SUM(amount_cents) OVER (PARTITION BY account ORDER BY ...)` restarts the total for each account and adds the rows in the window's order. The order has to be complete: `day` alone leaves same-day transactions tied, so add `id` after it.
---
Spell out the frame so each row stops at itself: `SUM(amount_cents) OVER (PARTITION BY account ORDER BY day, id ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS balance_cents`. Select `account, day, amount_cents` with it and finish with `ORDER BY account, day, id`.
