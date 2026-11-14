## La idea

El **diámetro** de un árbol es la longitud de su camino más largo, contada en aristas. Aquí el árbol es general (un nodo puede tener cualquier número de vecinos) y viene como una lista de padres: el padre del nodo `k` es el número `k`-ésimo, siempre un nodo menor, así que el nodo 0 es la raíz.

Un truco elegante solo necesita dos **búsquedas en anchura** (BFS), cada una `O(n)`:

1. Haz un BFS desde cualquier nodo, aquí el 0, y quédate con el nodo **más lejano** al que llega. Ese nodo siempre es un extremo de algún camino más largo.
2. Haz otro BFS desde ese extremo. La mayor distancia que alcanza es el diámetro.

En un árbol hay exactamente un camino entre dos nodos, así que las distancias de BFS son las reales.

### Por qué el primer extremo es correcto

Si el nodo más lejano `u` no fuera extremo de ningún camino más largo, se podría juntar el camino de 0 a `u` con parte de un camino más largo y obtener uno aún mayor, lo cual es imposible. El propio nodo 0, en cambio, puede quedar en medio.

### La entrada por defecto

Los padres 0, 1, 2, 3, 1, 2, 3, 0 dan 9 nodos: una columna 0 – 1 – 2 – 3 – 4 con una pata en 1, 2 y 3 (los nodos 5, 6 y 7) y el nodo 8 colgando de 0, con forma de oruga. El primer BFS desde 0 encuentra el nodo 4 a distancia 4 (el nodo 7 está igual de lejos pero se alcanza después). El segundo BFS desde 4 llega al nodo 8 a distancia 5, así que el diámetro es 5: el camino 4 – 3 – 2 – 1 – 0 – 8.

## Pasos

1. Construye las listas de vecinos: para cada nodo `k ≥ 1`, enlaza `k` con su padre en los dos sentidos.
2. BFS desde el nodo 0 con una cola, apuntando la distancia de cada nodo; guarda el más lejano, `end`.
3. Otro BFS desde `end`, apuntando las distancias desde él.
4. La mayor distancia del segundo BFS es el diámetro, en aristas.

## Coste

| Operación | Coste |
|---|---|
| Construir las listas de vecinos | `O(n)` |
| Cada BFS | `O(n)`: un árbol tiene `n − 1` aristas |
| Espacio extra | `O(n)`: las listas, las distancias y la cola |

## Errores comunes

- **Contar nodos en vez de aristas.** El camino 4 – 3 – 2 – 1 – 0 – 8 tiene 6 nodos pero 5 aristas. Mira cuál de las dos pide el enunciado.
- **Usar solo los enlaces a hijos.** El camino más largo a menudo sube y baja por otra rama; el BFS necesita enlaces en los dos sentidos.
- **Usarlo en grafos con ciclos.** El truco de los dos BFS se apoya en que un árbol tiene un solo camino entre nodos; en grafos generales puede dar una respuesta errónea.

## Cuándo usarlo

Usa el diámetro para preguntas de "cuánto pueden distar dos puntos" en un árbol: la ruta más larga en la red de senderos de un parque, el peor retraso entre dos máquinas de una red en árbol.

## Temas relacionados

- [Recorrido por niveles](learn:trees/level-order): BFS con una cola en un árbol binario.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): la cola sobre la que corre cada BFS.
- Búsqueda en anchura en grafos (próximamente): la misma búsqueda cuando a un nodo se llega por varios caminos.
