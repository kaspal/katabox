Sin el cupón es un problema de camino más barato con costes positivos en las casillas, que resuelve el algoritmo de Dijkstra. El cupón hace que el coste de una casilla dependa de una decisión tomada por el camino.
---
Haz que la decisión forme parte de la posición: el estado es `(casilla, ¿cupón usado?)`. Son dos copias de la cuadrícula. Entrar en una casilla cuesta su peaje y mantiene el estado del cupón; si el cupón sigue sin usar, también puedes entrar pagando la mitad del peaje y pasar a la copia "usado".
---
Ejecuta Dijkstra sobre esos 2 · filas · columnas estados con un montículo de mínimos. Empieza con `(0, 0)` sin usar a `tolls[0][0]` y usado a `tolls[0][0] // 2`. La respuesta es el más barato de los dos estados de la casilla de abajo a la derecha.
