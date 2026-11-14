## La idea

Algunos grafos solo tienen dos tipos de arista: las gratuitas (peso 0) y las que cuestan un paso (peso 1). Piensa en una cuadrícula donde andar por un pasillo es gratis pero romper una pared cuesta 1. BFS normal cuenta aristas, no peso; Dijkstra funciona, pero su cola de prioridad es más de lo que hace falta. **BFS 0-1** obtiene la misma respuesta con una **bicola** (una cola con dos extremos).

El ejemplo empieza en el nodo 0 con distancia 0. Saca el nodo `u` del frente de la bicola y mira cada arista `u – v` con peso `w`, en el orden de la lista de adyacencia. Si `v` todavía no tiene distancia, o `dist[u] + w` la mejora, `v` recibe la nueva distancia. Y aquí está el truco: una **arista 0** pone `v` al **frente**, porque está exactamente igual de lejos que `u` y debe tratarse a continuación; una **arista 1** pone `v` al **final**, un paso más lejos. Así la bicola se mantiene en orden: los nodos a distancia `d` al frente, los de distancia `d + 1` detrás y nunca nada más. Por eso, como en Dijkstra, la primera vez que un nodo sale de la bicola su distancia es definitiva.

A diferencia de BFS normal, un nodo puede entrar en la bicola **más de una vez**: primero se le puede alcanzar por una arista 1 y después mejorarlo con una arista 0. La copia posterior, cuando sale, ya no encuentra nada que mejorar.

### La entrada por defecto

Seis nodos, no dirigido: `0 1 1, 1 2 1, 2 5 1, 0 3 0, 3 4 0, 4 5 1, 3 1 1`. Desde 0, el nodo 1 cuesta 1 y va al final; el nodo 3 es gratis y va al frente, por delante de 1. Después sale el nodo 3 y pone 4 al frente gratis (distancia 0); su arista a 1 daría 0 + 1 = 1, que no mejora nada. El nodo 4 manda 5 al final con distancia 1. Luego sale 1 y da a 2 la distancia 2, y los nodos 5 y 2 no mejoran nada.

Los nodos salen de la bicola como 0, 3, 4, 1, 5, 2, y la salida es `[0, 1, 2, 0, 0, 1]`. El nodo 5 cuesta 1 por 0 → 3 → 4 → 5, aunque esa ruta tenga tres aristas.

## Pasos

1. Pon todas las distancias a `-1`, la del nodo de partida a `0`, y mete el nodo de partida en la bicola.
2. Saca el nodo `u` del frente.
3. Para cada arista `u – v` con peso `w`: si `v` es desconocido o `dist[u] + w < dist[v]`, pon `dist[v] = dist[u] + w` y añade `v` al frente si `w` es 0 y al final si es 1.
4. Repite hasta que la bicola esté vacía; los nodos no alcanzados siguen en `-1`.

## Coste

| Operación | Coste |
|---|---|
| Búsqueda completa | `O(n + m)`: las operaciones de la bicola son `O(1)` y cada nodo se fija una vez |
| Espacio extra | `O(n)` para las distancias y la bicola |

## Errores comunes

- **Marcar los nodos como vistos al descubrirlos, como hace BFS normal.** Un nodo encontrado por una arista 1 tiene que poder aceptar después una arista 0 más barata.
- **Meter las aristas 0 por el final.** La bicola pierde su orden y la primera salida de un nodo ya no es su distancia definitiva.
- **Una bicola lenta.** El ejemplo en TS usa `shift` y `unshift` sobre un arreglo, que desplazan todos los elementos; basta para una docena de nodos, pero el código real necesita una bicola de verdad.

## Cuándo usarlo

Usa BFS 0-1 cuando los movimientos son gratuitos o cuestan una unidad: el menor número de paredes que romper, de cambios de dirección o de calles de sentido único recorridas al revés. Con más de dos valores de peso, usa Dijkstra.

## Temas relacionados

- [Búsqueda en anchura (BFS)](learn:graphs/bfs): la misma idea cuando cada arista cuesta 1.
- [Caminos más cortos de Dijkstra](learn:graphs/dijkstra): la herramienta general para cualquier peso no negativo.
- [Bicola y cola monótona](learn:linear/deque-monotonic-queue): la cola de dos extremos que mueve la búsqueda.
- BFS en cuadrículas (próximamente): rutas más cortas sobre las celdas de una cuadrícula.
