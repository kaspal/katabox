## Enfoque: dos pasadas sobre el registro leído

El informe necesita primero el total, y decidir quién supera la media también lo necesita, así que lee todo el registro antes de escribir nada.

1. Lee toda la entrada y divídela en tokens: `N` y después parejas `nombre horas`. Los nombres también son tokens, porque nunca tienen espacios.
2. Suma las horas.
3. Recorre el registro en el orden de la entrada y quédate con cada voluntario con `horas > total / N`.

**Sin fracciones.** La media 49 / 5 = 9,8 no es entera. Multiplicando los dos lados por `N` (que es positivo) se obtiene la comparación equivalente `horas × N > total`, que solo usa enteros: sin redondeos de coma flotante, y los productos no pasan de 24 · 2·10⁴.

**Formato de salida.** Este problema se corrige línea a línea (salvo los espacios finales), así que la forma del informe importa: el total solo en la primera línea, un espacio entre cada nombre y sus horas, y la palabra `none` cuando la lista queda vacía. Junta las líneas y escríbelas de una vez.

**Por qué es correcto:** la comparación `horas × N > total` es exactamente «estrictamente por encima de la media», y el recorrido conserva el orden de la entrada.

**Complejidad:** tiempo y memoria O(N).

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python; decodifica los nombres antes de escribirlos), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` (PHP), `read_to_string` con `split_ascii_whitespace` y un `BufWriter` (Rust).
