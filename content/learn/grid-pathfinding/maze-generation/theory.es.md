## La idea

Las búsquedas de este tema necesitan laberintos sobre los que correr, y un buen laberinto se puede hacer crecer con una búsqueda en profundidad. Toma una cuadrícula cuadrada de lado `2k + 1`, toda muros. Las celdas con las dos coordenadas impares son **salas**, `k × k` en total; las celdas entre dos salas vecinas son los muros que se pueden derribar. El ejemplo abre la sala de la fila 1, columna 1 y la mete en una **pila**.

Cada ronda mira la sala de la cima de la pila y apunta las direcciones (arriba, derecha, abajo, izquierda) cuya sala a dos celdas de distancia está dentro de la cuadrícula y sigue siendo un muro, es decir, aún no se ha visitado. Si hay alguna, elige una al azar, abre el muro de en medio y esa sala, y mete la sala en la pila. Si no hay ninguna, la sala es por ahora un callejón sin salida y se saca: la búsqueda **retrocede** a la sala anterior. Cuando la pila se vacía, se han visitado todas las salas.

Cada sala abierta cuelga de exactamente una sala anterior, así que las aberturas forman un **árbol de expansión** de las `k²` salas: se quitan `k² − 1` muros, no hay ciclos y un único camino une dos salas cualesquiera. La salida es la cuadrícula fila a fila, `1` para muro y `0` para abierto.

Las elecciones al azar salen de un generador diminuto: `estado = estado × 48271 mod (2³¹ − 1)`, empezando por la semilla, y la elegida es `estado mod (número de opciones)`. El mismo `k` y la misma semilla dan siempre el mismo laberinto, así que un laberinto se puede compartir o repetir con dos números.

### La entrada por defecto

`k = 3`, semilla 7: una cuadrícula de 7 × 7 con nueve salas. El primer número es 7 × 48271 = 337897; desde la primera sala solo son posibles derecha y abajo, y 337897 es impar, así que elige la segunda: abajo. El laberinto terminado es:

```
#######
#.#...#
#.###.#
#...#.#
###.#.#
#.....#
#######
```

Han desaparecido ocho muros entre salas, así que hay 17 celdas abiertas. La sala de arriba a la izquierda llega a la de abajo a la derecha en ocho pasos (abajo, derecha, abajo, derecha por las salas), pero llegar a la sala de arriba a la derecha obliga a serpentear por las tres filas: los laberintos hechos en profundidad tienen pasillos largos.

## Pasos

1. Llena de muros una cuadrícula de `(2k + 1) × (2k + 1)`, abre la sala (1, 1) y métela en la pila.
2. Mira la sala de la cima. Apunta cada dirección cuya sala a dos pasos está dentro y sigue siendo un muro.
3. Si la lista no está vacía, elige `opciones[rand() mod cantidad]`, abre el muro de en medio y esa sala, y métela en la pila.
4. Si no, saca la sala de la cima.
5. Cuando la pila esté vacía, da la cuadrícula fila a fila.

## Coste

| Caso | Coste |
|---|---|
| Ejecución completa | `O(k²)`: cada sala entra en la pila una vez y sale una vez, revisando cuatro direcciones en cada ronda |
| Espacio extra | `O(k²)`: la cuadrícula y una pila que puede contener todas las salas |

## Errores comunes

- **Un estado a cero.** Si el estado llega a 0 se queda en 0 y siempre se elige la primera opción; las semillas empiezan en 1.
- **Desbordamiento.** El producto `estado × 48271` necesita más de 32 bits; los números de JavaScript lo guardan exacto (no pasa de 2⁴⁷), pero en un lenguaje con enteros de 32 bits usa enteros de 64.
- **Profundidad de recursión.** Una versión recursiva puede llegar a `k²` llamadas de profundidad; los ejemplos usan una pila explícita.

## Cuándo usarlo

Úsalo para crear niveles de juegos y rompecabezas, y mapas de prueba para búsquedas de caminos: un laberinto perfecto tiene una única ruta, así que todas las búsquedas tienen que encontrar la misma. Otros generadores dan laberintos con otro carácter: Prim aleatorizado hace crecer el laberinto desde una frontera de muros y deja muchos callejones cortos, y Kruskal aleatorizado quita muros en orden aleatorio, uniendo salas que aún no están conectadas.

## Temas relacionados

- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): el recorrido que excava el laberinto.
- [Pila](learn:linear/stack): la estructura que recuerda adónde retroceder.
- [Árbol de expansión mínima de Prim](learn:graphs/prim): su forma aleatorizada es otro generador de laberintos.
- [Árbol de expansión mínima de Kruskal](learn:graphs/kruskal): con unión-búsqueda, otra forma de hacer crecer un árbol de expansión de salas.
- [Búsqueda en anchura en una cuadrícula](learn:grid-pathfinding/grid-bfs): resuelve el laberinto que acabas de crear.
