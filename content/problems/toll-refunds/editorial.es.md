## Enfoque: Bellman–Ford con una comprobación final

1. `coste[start] = 0`, todos los demás ∞.
2. Repite n − 1 veces: para cada carretera `[a, b, c]` con `coste[a]` finito, haz `coste[b] = min(coste[b], coste[a] + c)`. Para antes si una ronda no cambia nada.
3. Una ronda más: si alguna carretera con `coste[a]` finito aún pudiera bajar `coste[b]`, se puede llegar a un circuito negativo desde `start`: devuelve `[]`.
4. Si no, devuelve los costes, escribiendo 10⁹ para los pueblos que siguen en ∞.

**Por qué n − 1 rondas:** sin un circuito negativo alcanzable, una ruta más barata nunca repite pueblo, así que tiene como mucho n − 1 carreteras; tras la ronda k, todo pueblo cuya ruta más barata tiene k carreteras ya es correcto.

**Por qué falla Dijkstra:** en el ejemplo 1 fijaría el pueblo 1 con coste 4 antes de descubrir 0 → 2 → 1 con coste 2.

**Complejidad:** O(n·m) en tiempo y O(n) de espacio: como mucho 1,5·10⁶ relajaciones aquí.
