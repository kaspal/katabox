Un usuario tiene todas las funciones de la pregunta `q` exactamente cuando `users[i] & q == q`: el número del usuario es un **superconjunto** de los bits de la pregunta. Comprobar cada usuario para cada pregunta son hasta 10¹⁰ comprobaciones.
---
Solo hay 2¹⁶ = 65 536 números posibles. Cuenta cuántos usuarios tienen cada número exacto y calcula una sola vez, para cada pregunta posible, el número de usuarios cuya configuración la contiene.
---
Sumas sobre superconjuntos: empieza con `cuenta[m]` = usuarios con exactamente `m`. Para cada bit `b` y cada `m` sin el bit `b`, suma `cuenta[m | b]` a `cuenta[m]`. Tras los 16 bits, `cuenta[m]` suma todos los superconjuntos de `m`. Cada pregunta es entonces una consulta a la tabla.
