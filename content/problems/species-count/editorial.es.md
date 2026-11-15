## Enfoque: preguntas ordenadas por el extremo derecho y marcas en la foto más reciente con un árbol de Fenwick

Reunir las especies de cada tramo en un conjunto cuesta O(r − l + 1) por pregunta: con 2·10⁴ preguntas que cubren casi todo un registro de 5·10⁴ fotos, son unos 8·10⁸ pasos. A diferencia de un mínimo o una suma, un recuento de valores distintos no se puede componer a partir de trozos precalculados, porque los trozos comparten especies. La salida es responder las preguntas **fuera de línea** (offline), en el orden que más nos convenga.

**La idea clave.** Ordena las preguntas por su extremo derecho `r` (o agrúpalas por `r`). Lee el registro de izquierda a derecha y, tras leer la foto `r`, mantén una marca en la foto **más reciente** de cada especie vista hasta ahora, y en ninguna otra. Para una pregunta `l r` a la que le ha llegado el turno:

- una especie que aparece en algún punto de `l..r` tiene su foto más reciente hasta `r` en esa aparición o después, así que dentro de `l..r`: tiene una marca ahí;
- una especie que no aparece en `l..r` no tiene ninguna foto ahí, así que tampoco una marca.

Así que la respuesta es simplemente el número de marcas en `l..r`. En el Ejemplo 1, tras la foto 6 (`12 5 12 9 5 5`), las fotos más recientes son la 3 (especie 12), la 4 (especie 9) y la 6 (especie 5); la pregunta `2 6` cuenta las marcas en 3, 4 y 6: tres especies.

**Mover las marcas.** Cuando la foto `r` muestra una especie cuya foto más reciente era `p`, quita la marca de `p` y marca `r`. Un mapa hash del identificador de especie a su foto más reciente encuentra `p`. Los identificadores llegan a 10⁹, así que usa un mapa (o comprime antes los identificadores), nunca un array indexado por identificador.

**Contar marcas rápido.** Un árbol de Fenwick sobre las posiciones `1..n` permite «sumar `d` en la posición `i`» y «contar las marcas en `1..k`» en O(log n) cada una:

```
add(i, d):  mientras i ≤ n:  tree[i] += d;  i += i & −i
count(k):   s = 0;  mientras k > 0:  s += tree[k];  k −= k & −k
```

La respuesta a `l r` es `count(r) − count(l − 1)`. Guárdala en el índice original de la pregunta y escribe las respuestas en el orden de la entrada al final.

**Por qué es correcto:** en el momento en que se responde una pregunta con extremo derecho `r`, están marcadas exactamente las fotos que son la más reciente de su especie entre las fotos `1..r` (cada paso mueve la única marca de una especie a su foto más nueva). Por los dos casos de arriba, las marcas de `l..r` se corresponden una a una con las especies de `l..r`.

**Complejidad:** tiempo O((n + Q) log n) y memoria O(n + Q).

**Notas prácticas:** lee todos los tokens de una vez y escribe todas las respuestas de una sola vez. En Python, los identificadores de especie pueden quedarse como cadenas de bytes: solo sirven de claves del mapa, así que no hace falta convertirlos. El `Number` de TypeScript representa cada identificador de forma exacta; los arrays de PHP funcionan como mapas hash con claves enteras; en Rust basta un `HashMap<u32, usize>`.
