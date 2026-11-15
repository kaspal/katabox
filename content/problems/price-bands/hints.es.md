Tienen que cumplirse dos condiciones a la vez: el producto está disponible y su precio está dentro de la franja. ¿Qué extremos de la franja quiere conservar la tienda?
---
Los dos extremos entran: 2000 y 4999 pasan, 1999 y 5000 no. `price_cents >= 2000 AND price_cents <= 4999` lo expresa, y también `price_cents BETWEEN 2000 AND 4999`, porque `BETWEEN` incluye sus dos extremos.
---
`WHERE in_stock = 1 AND price_cents BETWEEN 2000 AND 4999`, selecciona `name, price_cents` y termina con `ORDER BY price_cents DESC, name, id`.
