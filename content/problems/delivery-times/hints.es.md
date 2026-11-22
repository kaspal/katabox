Busca un punto que tenga una calle directa lenta desde el inicio pero una ruta más rápida pasando por otro punto. El ejemplo 1 tiene justo esa forma.
---
La construcción de las listas de calles y el propio montículo están bien. Céntrate en el momento en que se escribe la distancia de un punto en `dist`, y en si ese valor puede mejorarse después.
---
El código fija la distancia de un punto la primera vez que lo ve, por la calle por la que haya llegado. Una ruta más corta encontrada después se ignora porque `dist[u]` ya no es `-1`. En el algoritmo de Dijkstra una distancia solo es definitiva cuando el punto sale del montículo: salta los puntos extraídos que ya tienen distancia, si no asigna `dist[v] = d`, y mete los vecinos sin escribir su `dist`.
