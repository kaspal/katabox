Dijkstra supone que el coste de un pueblo ya no mejora cuando es el más barato sin visitar. Con reembolsos eso es falso: una carretera posterior puede abaratar un pueblo anterior.
---
Bellman–Ford: relaja todas las carreteras una y otra vez (`coste[b] = min(coste[b], coste[a] + c)`). Una ruta más barata sin circuitos usa como mucho n − 1 carreteras, así que bastan n − 1 rondas.
---
Tras n − 1 rondas, prueba una ronda más: si alguna carretera alcanzable aún puede bajar un coste, se puede llegar a un circuito negativo, así que devuelve una lista vacía. Relaja solo carreteras que salen de un pueblo ya alcanzado.
