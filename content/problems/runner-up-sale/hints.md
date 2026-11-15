Sorting a region's sales and taking the second row fails when two sales tie for the top. Rephrase the runner-up: it is the largest amount among the sales that are *below* the top amount.
---
For one region, the top is `(SELECT MAX(t.amount_cents) FROM sales t WHERE t.region = r.region)`. The runner-up is the `MAX` of the amounts `<` that top; when no amount is below it, `MAX` over no rows is `NULL`, which is exactly the answer you need.
---
Start from the list of regions, `FROM (SELECT DISTINCT region FROM sales) r`, so every region gets a row even without a runner-up. Select `r.region` and the correlated subquery above `AS runner_up_cents`, then `ORDER BY r.region`.
