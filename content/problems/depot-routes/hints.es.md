Todas las llamadas se atienden desde el mismo cruce, la base. ¿Y si calcularas la respuesta para cada cruce antes de mirar las llamadas?
---
Una sola ejecución del algoritmo de Dijkstra desde el cruce 1 encuentra el menor tiempo a todos los cruces a la vez, en O((N + M) log N) con un montículo binario. Lanzar una búsqueda nueva por cada llamada multiplica ese coste por `Q`.
---
Construye listas de adyacencia (cada calle entra en las listas de sus dos cruces) y ejecuta Dijkstra una vez desde el cruce 1 con un montículo de mínimos de pares `(tiempo, cruce)`, descartando los pares extraídos cuyo tiempo sea mayor que el mejor de su cruce. Guarda "infinito" para los cruces a los que no se llega; así cada llamada es una consulta de `dist[t]`, escribiendo `-1` si es infinito.
