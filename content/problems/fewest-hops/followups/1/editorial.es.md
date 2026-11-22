## Idea

El recorrido en anchura visita las estaciones por orden de distancia a la estación `0`, así
que cuando se saca de la cola una estación `u`, ya se han contado todas sus rutas más
cortas: cada una termina con un salto desde una estación un paso más cerca, y todas esas se
sacaron de la cola antes.

Guarda `dist` (−1 si no se ha visto) y `ways`, con `dist[0] = 0` y `ways[0] = 1`. Para cada
estación `u` que sale de la cola y cada vecino `v`:

- `dist[v] = −1`: primera vez que se ve. Fija `dist[v] = dist[u] + 1`, `ways[v] = ways[u]` y
  mete `v` en la cola.
- `dist[v] = dist[u] + 1`: otra forma más corta de llegar. Suma `ways[u]` a `ways[v]`.
- en otro caso el salto no está en ninguna ruta más corta; ignóralo.

La respuesta es `ways[n − 1]` (0 si nunca se llegó a `n − 1`). Cada suma se reduce módulo
1 000 003, así que los valores no pasan de 2·10⁶, sin peligro en ningún lenguaje.

## Complejidad

O(n + m) de tiempo para `m` enlaces y O(n + m) de memoria para las listas de adyacencia.
Enumerar las rutas, en cambio, es exponencial: una escalera de 2 estaciones por nivel tiene
2 elevado a su número de niveles.
