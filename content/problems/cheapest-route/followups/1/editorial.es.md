## Idea

El límite de carreteras rompe Dijkstra: la forma más barata de llegar a un pueblo puede
usar demasiadas carreteras para terminar a tiempo. Bellman-Ford cuenta carreteras de forma
natural. Sea `best[v]` el peaje más barato hasta `v` usando como mucho `r` carreteras; con
`r = 0` solo `start` cuesta 0. Una ronda prueba cada carretera `[a, b, t]` en los dos
sentidos: `next[b] = min(next[b], best[a] + t)` y lo mismo para `a`. Leer de `best` y
escribir en una copia `next` hace que cada ronda añada como mucho una carretera. Tras `k`
rondas, `best[goal]` es la respuesta (o `-1` si sigue siendo infinito). Si una ronda no
cambia nada, las siguientes tampoco, así que puedes parar antes.

## Complejidad

O(k · (n + m)) en tiempo para `m` carreteras (como mucho 50 · 10⁴ relajaciones por
sentido) y O(n) de memoria extra.
