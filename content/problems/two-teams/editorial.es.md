## Enfoque: colorear con dos colores mediante búsqueda en anchura

Los jugadores y las rivalidades forman un grafo. Un reparto válido es una coloración con dos colores en la que cada arista une colores distintos: el grafo debe ser **bipartito**.

Colorea el grafo componente a componente. Empieza un BFS desde cualquier jugador sin color con el color 0; cada rival de un jugador con color c recibe el color 1 − c. Si un rival ya tiene el color c, hay una arista entre dos jugadores que tendrían que estar a la vez en el mismo equipo y en equipos distintos (un ciclo impar), así que la respuesta es false. Si todas las componentes se colorean sin conflicto, la respuesta es true.

### Paso a paso

Ejemplo 2: el 0 recibe 0, y después el 1 y el 2 reciben 1. La rivalidad 1–2 une dos jugadores con color 1: false.

**Complejidad:** O(n + m) en tiempo y espacio. El BFS (o una pila explícita) evita una recursión profunda en cadenas largas.
