## Enfoque: tabla dispersa de mínimos por potencias de dos

Recorrer cada tramo cuesta O(N) por pregunta: con 5·10⁴ preguntas sobre un registro de 10⁵ lecturas son hasta 5·10⁹ pasos. El registro no cambia, así que conviene gastar O(N log N) una vez en una tabla que responda cualquier pregunta en O(1).

**La tabla.** Sea `low[k][i]` el mínimo de las `2^k` lecturas que empiezan en la posición `i` (desde 0). La fila 0 es el propio registro, y cada fila sale de la anterior combinando dos mitades:

```
low[k][i] = min(low[k−1][i], low[k−1][i + 2^(k−1)])     para 0 ≤ i ≤ N − 2^k
```

Hay ⌊log₂ N⌋ + 1 ≤ 17 filas, cada una de longitud como mucho `N`.

**Una pregunta.** Para el tramo `l..r` (desde 0) de longitud `len = r − l + 1`, toma el mayor `k` con `2^k ≤ len`. Los dos trozos `[l, l + 2^k)` y `(r − 2^k, r]` están dentro del tramo y juntos lo cubren (cada uno es al menos la mitad), así que

```
respuesta = min(low[k][l], low[k][r − 2^k + 1])
```

**Por qué el solapamiento no importa:** los dos trozos pueden compartir lecturas, pero tomar un mínimo dos veces no cambia nada (`min` es idempotente). Por eso la tabla dispersa sirve para mínimo, máximo o mcd, y no para sumas, donde el solapamiento contaría lecturas dos veces.

**Cómo hallar k:** `len.bit_length() − 1` (Python), `31 − Math.clz32(len)` (TypeScript), una tabla precalculada `lg[i] = lg[i / 2] + 1` (PHP), `usize::BITS − 1 − len.leading_zeros()` (Rust).

**Complejidad:** tiempo y memoria O(N log N) para construirla, O(1) por pregunta: O(N log N + Q) en total. Recuerda que las posiciones de la entrada empiezan en 1.

**Entrada y salida rápidas:** lee todos los tokens de una vez y escribe todas las respuestas de una sola vez. En Python, construye cada fila con cortes al estilo `list(map(min, prev[:-half], prev[half:]))` para que el bucle corra en C.
