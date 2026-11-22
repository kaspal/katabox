## Idea

Duplica el grafo en dos capas. Guarda dos distancias por pueblo: `dist[v][0]` con el pase
sin usar y `dist[v][1]` con el pase gastado. Empieza con `dist[start][0] = 0` y ejecuta
Dijkstra sobre esos 2·n estados. Salir de un estado sin usar por una carretera
`[u, v, t]` da dos movimientos: pagar `t` y seguir sin usar, o gastar el pase y llegar a
`dist[v][1]` sin coste. Desde un estado gastado, cada carretera cuesta su peaje. Todos los
pesos son no negativos, así que Dijkstra es correcto, y la respuesta es
`min(dist[goal][0], dist[goal][1])`, o `-1` si los dos son infinitos.

## Complejidad

O((n + m) log(n + m)) en tiempo para `m` carreteras con un montículo binario, y O(n + m)
de memoria extra para las listas de adyacencia y el montículo.
