## Enfoque: sumas prefijas de los tramos y búsqueda binaria

Una etiqueta como `1000000000x5y999999999z` mide dos mil millones de letras, así que construir la cadena desempaquetada es imposible. Recorrer el texto empaquetado tramo a tramo en cada revisión funciona, pero cuesta O(tramos) por revisión: con 60 000 tramos y 20 000 revisiones son más de 10⁹ pasos.

Analiza cada etiqueta **una sola vez** y guarda dos arreglos:

- `letras[r]`, la letra del tramo `r`;
- `finales[r]`, la posición de la última letra del tramo `r`: la suma de las cantidades de los tramos `0 .. r`.

Para `3a2b10c`: `letras = [a, b, c]`, `finales = [3, 5, 15]`. El tramo `r` cubre las posiciones `finales[r-1] + 1 .. finales[r]`, así que la posición `k` está en el **primer tramo con `finales[r] ≥ k`**. Cada cantidad es al menos 1, de modo que `finales` es estrictamente creciente y la búsqueda binaria encuentra ese tramo en O(log tramos). En el ejemplo, `k = 4` da `r = 1`, la letra `b`.

Dos tramos seguidos pueden compartir letra (`2k1k3m`); no cambia nada, porque la respuesta es simplemente la letra del tramo encontrado.

**Lectura de la entrada:** divide toda la entrada por espacios; los tokens son `L`, las `L` etiquetas, `Q` y después los pares `i k`. Para analizar una etiqueta, acumula los dígitos en una cantidad y cierra el tramo en cada letra. Cada cantidad es como mucho 10⁹ y una etiqueta tiene como mucho 60 000 tramos, así que todo final queda por debajo de 6·10¹³ < 2⁵³: un `Number` normal es exacto en TypeScript, y un `int` en PHP.

**Complejidad:** tiempo O(longitud total de las etiquetas + Q · log tramos), memoria O(tramos en total).

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` con `re.findall` y `bisect_left` (Python), `readFileSync(0, "utf8").split(/\s+/)` y `charCodeAt` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` y `preg_match_all` (PHP), `read_to_string` con `split_ascii_whitespace` y `partition_point` (Rust).
