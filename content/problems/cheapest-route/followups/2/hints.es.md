Dejar gratis la carretera más cara de la ruta más barata normal no siempre es lo mejor:
una ruta peor sin el pase puede ser mejor con él. ¿Qué más necesitas saber al llegar a un
pueblo?
---
En cada pueblo, el mensajero está en una de dos situaciones: aún no ha usado el pase, o ya
lo ha gastado. Trata cada par (pueblo, situación) como un nodo propio.
---
Ejecuta Dijkstra sobre 2·n estados. Desde un estado sin usar, una carretera lleva al
estado sin usar del siguiente pueblo por su peaje, o a su estado gastado por 0. Desde un
estado gastado, las carreteras cuestan su peaje. La respuesta es el menor de los dos
estados en `goal`.
