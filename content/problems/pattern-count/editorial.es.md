## Enfoque: Knuth–Morris–Pratt

**Función de prefijos.** Para el patrón `p`, sea `pi[i]` la longitud del prefijo propio más largo de `p[0..i]` que también es sufijo suyo. Se construye de izquierda a derecha: con `k = pi[i − 1]`, mientras `k > 0` y `p[i] ≠ p[k]`, retrocede `k = pi[k − 1]`; si `p[i] = p[k]`, `k++`; después `pi[i] = k`.

**Recorrido.** Lleva `k`, cuántos caracteres del patrón coinciden ahora con el final del texto leído. Para cada carácter `c` del texto: mientras `k > 0` y `c ≠ p[k]`, `k = pi[k − 1]`; si `c = p[k]`, `k++`. Si `k` es igual a la longitud del patrón, aquí termina una aparición: cuéntala y pon `k = pi[k − 1]`, para encontrar también las apariciones solapadas.

`k` crece como mucho uno por carácter y cada retroceso lo reduce, así que el trabajo total es lineal.

### Paso a paso

`pattern = "tat"` tiene `pi = [0, 0, 1]`. En "tatatat", la coincidencia llega a 3 en las posiciones 2, 4 y 6; cada vez `k` retrocede a `pi[2] = 1` (la `t` final puede empezar la siguiente coincidencia).

**Complejidad:** O(n + m) en tiempo y O(m) de espacio. Comparar en cada posición es O(n · m), 10¹⁰ pasos con un texto de 2·10⁵ `a` y un patrón de 10⁵.
