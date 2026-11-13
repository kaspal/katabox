Dibuja los jugadores como puntos y las rivalidades como líneas. En cuanto pones a un jugador en el equipo A, el equipo de cada jugador conectado con él queda forzado.
---
Da a cada jugador un color (equipo 0 o equipo 1) recorriendo el grafo: un rival de un jugador recibe el otro color.
---
Para cada jugador aún sin color, empieza un BFS con el color 0. Si ves un rival que ya tiene el **mismo** color que el jugador actual, no existe el reparto.
