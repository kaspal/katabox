Ordenar las cartas como números (76 < 760) o como texto ("76" < "760") falla en algún caso. Mira solo dos cartas: ¿cuál debe ir primero?
---
La carta `a` va antes que la `b` exactamente cuando el texto `a + b` es mayor que `b + a` (los dos tienen la misma longitud, así que compáralos como cadenas).
---
Ordena con esa comparación, une los textos y convierte un resultado formado solo por ceros en "0".
