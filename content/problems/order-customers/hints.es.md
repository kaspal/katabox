Los importes están en `orders`; los nombres y las ciudades, en `customers`. ¿Qué columna de un pedido dice quién lo hizo?
---
`JOIN customers c ON c.id = o.customer_id` pone cada pedido junto a su cliente. No unas por `o.id`: es el número del propio pedido y puede coincidir con el id de un cliente que no tiene nada que ver. Quédate con las filas con `o.total_cents >= 10000`; un pedido de 100,00 justos cuenta.
---
Selecciona `o.id AS order_id, c.name, c.city, o.total_cents` y termina con `ORDER BY o.total_cents DESC, o.id`, para que los importes iguales salgan por id de pedido.
