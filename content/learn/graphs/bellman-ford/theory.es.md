## La idea

**Bellman-Ford** encuentra la ruta más barata desde el nodo 0 hasta cada nodo de un grafo dirigido y, a diferencia de Dijkstra, admite pesos **negativos**. No elige un orden ingenioso. Simplemente recorre toda la lista de aristas una y otra vez, relajando cada arista `u → v` con peso `w`: si `u` ya se ha alcanzado y `dist[u] + w` mejora `dist[v]`, baja `dist[v]`.

¿Por qué bastan `n − 1` rondas? Un camino más barato que no da vueltas visita cada nodo como mucho una vez, así que tiene como mucho `n − 1` aristas. Tras la ronda 1, cada nodo cuyo camino más barato tiene una arista ya tiene su distancia definitiva; tras la ronda `k`, cada nodo cuyo camino tiene como mucho `k` aristas. Cada ronda hace definitiva al menos una arista más de cada camino, así que `n − 1` rondas los fijan todos. El ejemplo para antes tras una ronda en la que no cambió nada, porque las siguientes tampoco cambiarían nada.

El **orden** de la lista de aristas decide cuánto avanza una ronda: si las aristas de un camino aparecen en el orden del camino, una sola ronda lleva la distancia de punta a punta; en orden inverso, cada ronda la mueve una arista. Las rondas cambian, pero las distancias finales no.

Si desde 0 se alcanza un **ciclo negativo**, dar vueltas por él baja el coste sin fin, y no existe respuesta. Una pasada extra por las aristas lo detecta: si alguna arista todavía mejora una distancia tras `n − 1` rondas, el ejemplo escribe `[1]`. Si no, escribe `[0]` seguido de las distancias, donde `999` significa inalcanzable.

### La entrada por defecto

Cinco nodos, aristas `3→4 2, 1→3 −2, 0→1 4, 0→2 1, 2→1 2, 2→3 5`. La ronda 1 se salta las dos primeras aristas (3 y 1 aún no se han alcanzado), después pone 1 a 4 y 2 a 1, mejora 1 a 3 pasando por 2 y pone 3 a 6. La ronda 2 pone 4 a 8 y luego baja 3 a 1 por la arista negativa. La ronda 3 baja 4 a 3. La ronda 4 no cambia nada, así que el bucle para, y la pasada de comprobación no encuentra ninguna mejora.

La salida es `[0, 0, 3, 1, 1, 3]`: no hay ciclo negativo, y las distancias son 0, 3, 1, 1, 3. El camino hasta 4 es 0 → 2 → 1 → 3 → 4, cuatro aristas, y por eso necesitó varias rondas.

## Pasos

1. Pon `dist[0] = 0` y todas las demás distancias a `999`.
2. Repite hasta `n − 1` veces: relaja cada arista en el orden de la lista; para antes si ninguna distancia cambió.
3. Haz una pasada más: si alguna arista todavía mejora una distancia, escribe `[1]`.
4. Si no, escribe `[0]` y las distancias.

## Coste

| Operación | Coste |
|---|---|
| Mejor caso | `O(m)` cuando una ronda lo fija todo |
| Caso medio y peor | `O(n · m)`: hasta `n − 1` rondas sobre `m` aristas |
| Espacio extra | `O(n)` para las distancias |

## Errores comunes

- **Relajar desde un nodo no alcanzado.** `999 + (−2)` parecería una distancia real; sáltate las aristas cuyo origen aún es inalcanzable.
- **Leer las distancias después de un ciclo negativo.** No significan nada; comprueba primero.

## Cuándo usarlo

Úsalo cuando los pesos pueden ser negativos: reembolsos, cambio de divisas o diferencias entre restricciones. Si todos los pesos son no negativos, Dijkstra es mucho más rápido.

## Temas relacionados

- [Caminos más cortos de Dijkstra](learn:graphs/dijkstra): más rápido, pero solo con pesos no negativos.
- [Detección de ciclos](learn:graphs/cycle-detection): encontrar cualquier ciclo en un grafo dirigido.
- Floyd-Warshall (próximamente): caminos más cortos entre cada par de nodos.
