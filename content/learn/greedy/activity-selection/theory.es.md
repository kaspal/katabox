## La idea

Tienes una lista de actividades, cada una con una hora de inicio y otra de fin, y una sola sala. Dos actividades chocan si una empieza antes de que termine la otra; tocarse está permitido, así que una actividad que acaba a las 6 y otra que empieza a las 6 caben las dos. ¿Cuál es el **mayor número** de actividades que caben?

Un algoritmo **voraz** construye su respuesta elección a elección y nunca deshace ninguna. Aquí la elección es: entre las actividades que aún son posibles, toma la que **termina antes**. El ejemplo ordena las actividades por su fin (si empatan, por su inicio y después por su posición en la entrada), las recorre una vez y toma cada una que empieza no antes de que termine la última tomada. La salida es el número de actividades y después las posiciones elegidas (empezando en 0) en orden creciente. La entrada se escribe como pares "inicio fin".

¿Por qué es seguro el fin más temprano? Toma cualquier calendario óptimo y mira su primera actividad. Cámbiala por la que termina antes: esa acaba no más tarde, así que no puede chocar con nada de lo que venía después. El número no cambia, y lo que queda es el mismo problema con una lista más corta. Otras reglas fallan con ejemplos diminutos. **La más corta primero** falla con `(0, 5)`, `(4, 7)`, `(6, 11)`: toma `(4, 7)`, que choca con las otras dos, y consigue 1 en vez de 2. **La que empieza antes** falla con `(0, 10)`, `(1, 3)`, `(4, 6)`: la larga va primero y bloquea las dos cortas.

Un problema se presta a un algoritmo voraz cuando tiene una **elección voraz** (alguna elección local forma parte siempre de una respuesta óptima) y **subestructura óptima** (tras esa elección, lo que queda es una copia más pequeña del mismo problema). Desconfía de toda idea voraz hasta haber intentado romperla con un ejemplo pequeño de dos o tres elementos. Si encuentras uno que la rompe, el problema probablemente necesita programación dinámica.

### La entrada por defecto

Ocho actividades: `(1, 4)`, `(3, 5)`, `(0, 10)`, `(5, 7)`, `(4, 6)`, `(6, 9)`, `(8, 11)`, `(9, 12)`, posiciones 0 a 7. Por su fin, el orden es posiciones 0, 1, 4, 3, 5, 2, 6, 7. Se toma la posición 0 (libre a las 4). La posición 1 empieza a las 3: choque. La posición 4 empieza a las 4: se toma (libre a las 6). La posición 3 choca y la posición 5 empieza a las 6: se toma (libre a las 9). Las posiciones 2 y 6 chocan, y la posición 7 empieza a las 9: se toma.

La salida es `[4, 0, 4, 5, 7]`. La regla de la que empieza antes se quedaría con la reunión larga `(0, 10)` y no le cabría nada más.

## Pasos

1. Ordena las posiciones por fin, después por inicio y después por posición.
2. Guarda `free_at`, el fin de la última actividad tomada (ninguno al principio).
3. Para cada actividad en ese orden: si `inicio ≥ free_at`, tómala y pon `free_at` a su fin; si no, sáltala.
4. Devuelve el número y las posiciones tomadas, ordenadas.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n log n)`: la ordenación; el recorrido en sí es `O(n)` |
| Espacio extra | `O(n)` para el orden y la lista de elegidas |

## Errores comunes

- **Ordenar por inicio.** Parece natural, pero deja que una actividad larga bloquee muchas cortas.
- **Estricto o no.** Aquí tocarse está permitido (`inicio ≥ free_at`); si un problema exige un hueco entre actividades, usa `>`.
- **Actividades con peso.** Cuando cada actividad tiene un valor y el objetivo es el mayor valor total, el fin más temprano deja de ganar; esa versión necesita programación dinámica.

## Cuándo usarlo

Úsalo para meter el mayor número de reuniones, charlas o tareas en una sala o una máquina. El problema enlazado, El mayor número de charlas, sigue este patrón.

## Temas relacionados

- [Fusionar intervalos](learn:greedy/merge-intervals): los mismos intervalos ordenados, unidos en vez de elegidos.
- [Cambio de monedas](learn:dynamic-programming/coin-change): una regla voraz que falla y la tabla que lo arregla.
- [Mochila 0/1](learn:dynamic-programming/knapsack-01): donde "primero el mejor valor por peso" falla.
- [Búsqueda voraz primero el mejor](learn:grid-pathfinding/greedy-best-first): una búsqueda voraz rápida pero no siempre la más barata.
