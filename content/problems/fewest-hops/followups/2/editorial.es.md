## Idea

Busca en un grafo de dos capas. El estado `(v, 0)` significa "en la estación `v`, sin
ningún enlace girado todavía" y `(v, 1)` significa "en la estación `v`, con el giro
gastado". Cada enlace `[a, b]` da las aristas `(a, 0) → (b, 0)` y `(a, 1) → (b, 1)`, y
además `(b, 0) → (a, 1)`: cruzarlo al revés es el momento en que lo giras. Una búsqueda en
anchura desde `(0, 0)` da el menor número de saltos a cada estado; la respuesta es la
menor de las distancias a `(n − 1, 0)` y `(n − 1, 1)` que existan, o `-1` si no se alcanzó
ninguna.

¿Podría la búsqueda usar un enlace en su sentido y también girado, cosa que las reglas
prohíben? Una ruta que cruza el mismo enlace en los dos sentidos visita una estación dos
veces, y una ruta más corta nunca lo hace, así que las distancias encontradas siempre se
pueden lograr con un giro de verdad.

## Complejidad

O(n + links.length) en tiempo y memoria: el grafo por capas tiene 2n estados y 3 aristas
por enlace.
