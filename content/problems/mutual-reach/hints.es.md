Los grupos son las componentes fuertemente conexas del grafo dirigido. Comprobar el alcance desde cada usuario por separado es O(n·(n + m)).
---
La idea de Kosaraju: un DFS sobre el grafo ordena los usuarios por tiempo de finalización. Después, un DFS sobre el grafo **invertido**, tomando los usuarios de inicio en orden decreciente de finalización, reúne exactamente un grupo por cada inicio.
---
Haz las dos pasadas de forma iterativa (con pilas explícitas): 10⁵ usuarios en cadena desbordarían un DFS recursivo. Cuenta los tamaños y ordénalos.
