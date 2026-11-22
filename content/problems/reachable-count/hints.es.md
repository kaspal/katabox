Ejecuta el código a mano con el mapa más pequeño que tenga una puerta: `n = 2, doors = [[0, 1]], start = 0`. ¿Qué salas pasan por la cola y cuántas veces cada una?
---
Cada sala debería entrar en la cola exactamente una vez. Fíjate en qué salas se marcan en `seen` y en qué momento.
---
`start` entra en la cola sin marcarse como vista, así que su primer vecino la vuelve a encolar y se cuenta dos veces. Pon `seen[start] = True` (`seen[start] = true` en TypeScript) antes del bucle.
