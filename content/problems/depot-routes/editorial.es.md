## Enfoque: un solo Dijkstra y luego consultas

Todas las llamadas piden un camino mínimo **desde el mismo cruce**, la base. Una búsqueda de caminos mínimos desde el cruce 1 las responde todas a la vez, así que se ejecuta una vez, se guarda el tiempo de cada cruce y cada llamada se responde con una consulta.

Todos los tiempos de las calles son positivos, así que sirve el **algoritmo de Dijkstra**:

1. Construye listas de adyacencia: una calle `a b w` añade `(b, w)` al cruce `a` y `(a, w)` al cruce `b`. Las calles paralelas no necesitan nada especial; la más rápida gana sola.
2. Pon `dist[1] = 0` y el resto de cruces a infinito; mete `(0, 1)` en un montículo de mínimos.
3. Saca el par más pequeño `(d, u)`. Si `d > dist[u]`, es una entrada obsoleta: descártala. Si no, para cada calle `(v, w)` que sale de `u`, si `d + w < dist[v]`, pon `dist[v] = d + w` y mete `(d + w, v)`.
4. Cuando el montículo se vacía, `dist[t]` es la respuesta para el cruce `t`, o `-1` si sigue siendo infinito.

**Por qué es correcto:** con tiempos positivos, la primera vez que un cruce sale con un tiempo no obsoleto ya no puede existir una ruta más corta hasta él: cualquier otra ruta abandona el conjunto de cruces terminados por un cruce cuyo tiempo ya es al menos igual. Los cruces que nunca entran en el montículo no tienen ruta desde la base.

**Por qué una sola vez:** una búsqueda por llamada cuesta O(Q·(N + M) log N), unos 10⁹ pasos de montículo en los límites. Una sola búsqueda cuesta O((N + M) log N) y cada llamada O(1).

**Montículos en cada lenguaje:** Python tiene `heapq` y Rust `BinaryHeap` (envuelve los pares en `Reverse` para tener un montículo de mínimos). TypeScript no tiene montículo: escribe uno binario pequeño sobre dos arrays paralelos (tiempos y cruces) con subida y bajada. `SplPriorityQueue` de PHP es un montículo de máximos, así que inserta cada cruce con el tiempo **negado** como prioridad.

**Complejidad:** tiempo O((N + M) log N + Q), memoria O(N + M). El mayor tiempo posible es (N − 1)·10⁴ < 10⁸, así que bastan enteros normales en todas partes.

**Entrada rápida:** lee todo de una vez y recorre los tokens con una posición (`sys.stdin.buffer.read().split()`, `readFileSync(0, "utf8")`, `stream_get_contents(STDIN)`, `read_to_string`), y escribe todas las respuestas de una sola vez.
