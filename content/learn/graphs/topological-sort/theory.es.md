## La idea

Un **orden topológico** pone en fila los nodos de un grafo dirigido de forma que cada arista `u → v` apunte hacia delante: `u` va antes que `v`. Piensa en tareas y sus requisitos previos: un orden válido es un calendario en el que nada empieza antes que aquello de lo que depende.

El **algoritmo de Kahn** construye el orden desde el principio. El **grado de entrada** de un nodo es el número de aristas que apuntan a él. Un nodo con grado de entrada 0 no espera a nadie, así que puede ir primero. Mete todos esos nodos en una cola FIFO, en orden de índice. Saca uno, escríbelo y quita sus aristas de salida bajando en uno el grado de entrada de cada destino. Todo destino que baja a 0 acaba de perder su último requisito y entra por el final de la cola.

Si el grafo tiene un **ciclo**, sus nodos nunca llegan a grado de entrada 0: cada uno espera al anterior del bucle, y ninguno puede ir primero. Nunca entran en la cola, así que el orden sale **más corto que `n`**. Comparar su longitud con `n` es una comprobación de ciclos gratis.

### La entrada por defecto

Siete nodos, dirigido, con las aristas `0→2, 1→2, 1→3, 2→4, 3→4, 4→5, 3→6`. Los grados de entrada son 0, 0, 2, 1, 2, 1, 1, así que la cola empieza con 0, 1. Sacar 0 baja 2 a 1. Sacar 1 baja 2 a 0 y 3 a 0, así que los dos entran en la cola. Sacar 2 baja 4 a 1; sacar 3 baja 4 a 0 y 6 a 0. Después 4 libera a 5, y la cola entrega 6 antes que 5 porque 6 entró antes.

La salida es `0, 1, 2, 3, 4, 6, 5`: los siete nodos, así que el grafo no tiene ciclos.

## Pasos

1. Cuenta el grado de entrada de cada nodo recorriendo cada arista una vez.
2. Mete en la cola cada nodo con grado de entrada 0, en orden de índice.
3. Mientras la cola no esté vacía, saca el nodo `u` del frente y escríbelo.
4. Para cada arista `u → v`, baja en uno el grado de entrada de `v`; si llega a 0, añade `v` al final de la cola.
5. Si se han escrito menos de `n` nodos, el grafo tiene un ciclo.

## Coste

| Operación | Coste |
|---|---|
| Ordenación completa | `O(n + m)`: los grados de entrada cuestan una pasada por las aristas, y después cada nodo y cada arista se tratan una vez |
| Espacio extra | `O(n)` para los grados de entrada y la cola |

## Errores comunes

- **Esperar una sola respuesta.** Un grafo suele tener muchos órdenes topológicos; este depende de las reglas de la cola. Usa una cola de prioridad si necesitas el orden más pequeño por índice.
- **Ignorar un resultado corto.** Devolver el orden parcial como si estuviera completo oculta el ciclo; comprueba la longitud.
- **Olvidar los nodos sin aristas.** Tienen grado de entrada 0 y también deben estar en la cola inicial.

## Cuándo usarlo

Úsalo para planificar cualquier cosa con dependencias: pasos de compilación, planes de estudio, el recálculo de una hoja de cálculo o la programación dinámica sobre un grafo dirigido acíclico, donde cada nodo necesita antes las respuestas de sus predecesores. La alternativa es DFS: escribe cada nodo cuando termina y luego invierte esa lista. También da un orden válido, pero la versión de Kahn detecta ciclos contando y no necesita recursión.

## Temas relacionados

- [Detección de ciclos](learn:graphs/cycle-detection): la forma con DFS de demostrar que existe un orden.
- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): su orden de finalización invertido también es topológico.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): la cola de nodos listos.
- [Montículo binario y cola de prioridad](learn:trees/binary-heap): para el orden más pequeño por índice.
