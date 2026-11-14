## Enfoque: contar una vez y responder consultando

**Leer líneas de formas distintas.** La entrada tiene tres tipos de líneas: dos números, una fila de caracteres del mapa y una palabra con un número. Divide toda la entrada por espacios y recorre los tokens con una posición:

1. `R`, `C`;
2. los `R` tokens siguientes son las filas del mapa (una fila nunca contiene espacios, así que es un único token);
3. `Q`;
4. después, `Q` parejas `tipo índice`.

**Precalcular.** Responder cada pregunta recorriendo su fila o su columna cuesta hasta 300 pasos, es decir, hasta 6·10⁶ pasos en total: aceptable en un lenguaje compilado, lento en uno interpretado y, en cualquier caso, trabajo repetido, porque el mapa no cambia. En su lugar, recorre el mapa una vez: por cada `#` en `(i, j)`, suma 1 a `arbolesFila[i]` y a `arbolesColumna[j]`. Así cada pregunta es una sola consulta en un arreglo.

**Por qué es correcto:** cada árbol se cuenta una vez en el total de su fila y una vez en el de su columna, que es justo lo que pregunta cada consulta.

**Complejidad:** tiempo O(R·C + Q) y memoria extra O(R + C).

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python; `fila.count(b"#")` cuenta una fila y `zip(*filas)` da las columnas), `readFileSync(0, "utf8").split(/\s+/)` con `charCodeAt` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` y `substr_count` (PHP), `read_to_string` con `split_ascii_whitespace`, `as_bytes()` y un `BufWriter` (Rust). Junta las respuestas y escríbelas de una vez.
