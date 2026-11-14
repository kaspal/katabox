## La idea

Un grafo es **bipartito** cuando sus nodos se pueden repartir en dos lados, A y B, de forma que cada arista une un lado con el otro y ninguna arista se queda dentro de un lado.

Para comprobarlo, intenta colorear los nodos. El ejemplo lanza un BFS desde cada nodo que aún no tiene lado: el nodo de partida va al lado A (`0`), y cada vecino recién descubierto va al lado contrario del nodo que lo encontró (`1 − side`). A lo largo de cada camino que sigue BFS, los lados simplemente se alternan, así que el lado de un nodo indica si está a un número par o impar de pasos del inicio. Una arista cuyos dos extremos ya están en el **mismo lado** significa que hay dos rutas entre esos extremos, una de longitud par y otra de longitud impar, y juntas cierran un **ciclo impar**. Un ciclo impar nunca puede alternar en toda la vuelta, así que la respuesta es no y el ejemplo escribe `[-1]`. Si no, escribe el lado de cada nodo.

El bucle exterior sobre todos los nodos importa: cada componente se colorea por separado, y un grafo es bipartito solo si lo son todas sus componentes.

### La entrada por defecto

Ocho nodos, no dirigido: un cuadrado 0 – 1 – 2 – 3 – 0, una arista 2 – 4 y un segundo cuadrado 4 – 5 – 6 – 7 – 4. El nodo 0 va a A, y visitarlo pone 1 y 3 en B. Visitar 1 pone 2 en A; visitar 3 encuentra 2 en A y 0 en A, los dos en el lado contrario a B, así que no hay choque. Después 2 pone 4 en B, 4 pone 5 y 7 en A, y 5 pone 6 en B. Todas las comprobaciones que quedan encuentran lados opuestos.

La salida es `[0, 1, 0, 1, 1, 0, 1, 0]`: el lado A es {0, 2, 5, 7} y el lado B es {1, 3, 4, 6}. Los dos cuadrados son ciclos de longitud 4, que es par, así que alternan sin problema.

## Pasos

1. Da a cada nodo el lado `-1` (ninguno todavía).
2. Para cada nodo `s` sin lado, en orden de índice: ponlo en el lado `0` y empieza con él una cola de BFS.
3. Saca el nodo `u` del frente. Para cada vecino `v`: si `v` no tiene lado, dale `1 − side[u]` y encólalo.
4. Si `v` ya tiene el mismo lado que `u`, para y escribe `[-1]`.
5. Cuando todas las componentes están coloreadas, escribe los lados.

## Coste

| Operación | Coste |
|---|---|
| Comprobación completa | `O(n + m)`: cada nodo entra en la cola una vez y cada lista se lee una vez |
| Espacio extra | `O(n)` para los lados y la cola |

## Errores comunes

- **Colorear solo desde el nodo 0.** Un choque en una componente a la que 0 no llega pasaría desapercibido.
- **Tomar "mismo lado" como algo inofensivo.** Una arista hacia un nodo ya coloreado solo está bien si su lado es el contrario; un lado igual es toda la prueba del fallo.
- **Esperar un reparto único.** Cada componente se puede invertir, A por B, por separado; la respuesta del ejemplo es la que pone el nodo más pequeño de cada componente en el lado A.

## Cuándo usarlo

Úsalo para dividir a personas en dos equipos con todas las rivalidades entre equipos, para comprobar si un grafo de conflictos de horario se puede cubrir con dos franjas, o como primer paso antes del emparejamiento bipartito, que empareja un lado con el otro.

## Temas relacionados

- [Búsqueda en anchura (BFS)](learn:graphs/bfs): el recorrido que extiende los colores.
- [Componentes conexas](learn:graphs/connected-components): el mismo bucle exterior sobre cada trozo.
- [Detección de ciclos](learn:graphs/cycle-detection): encontrar cualquier ciclo, no solo los impares.
- Flujo máximo (próximamente): emparejar los dos lados de un grafo bipartito.
