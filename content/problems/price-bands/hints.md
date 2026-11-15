Two conditions must hold at once: the product is in stock, and its price is inside the band. Which end values of the band does the shop want to keep?
---
Both ends are in: 2000 and 4999 pass, 1999 and 5000 do not. `price_cents >= 2000 AND price_cents <= 4999` says that, and so does `price_cents BETWEEN 2000 AND 4999`, because `BETWEEN` includes both of its ends.
---
`WHERE in_stock = 1 AND price_cents BETWEEN 2000 AND 4999`, select `name, price_cents`, and finish with `ORDER BY price_cents DESC, name, id`.
