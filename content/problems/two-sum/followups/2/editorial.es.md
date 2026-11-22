## Idea

Un mapa hash responde "¿está la pareja exacta?", no "¿cuál es la pareja más cercana?",
así que conviene ordenar. Con un puntero `i` al principio y otro `j` al final, mira
`s = a[i] + a[j]`. Si `s < target`, cualquier otra pareja de `a[i]` es menor que `a[j]`
y da una suma aún más pequeña, así que `a[i]` ya no sirve: avanza `i`. Si
`s > target`, `a[j]` queda descartado del mismo modo: retrocede `j`. Si
`s == target`, nada puede estar más cerca. Cada suma que los punteros saltan está más
lejos de `target` que alguna que sí revisan, así que quedarse con la mejor revisada (en
empate, la menor) da la respuesta.

## Complejidad

O(n log n) en tiempo por la ordenación y después O(n) por el recorrido; O(n) de memoria
extra para una copia ordenada (u O(1) si ordenas en el sitio). Las sumas llegan a
2·10⁹, más de lo que cabe en un entero de 32 bits: usa aritmética de 64 bits.
