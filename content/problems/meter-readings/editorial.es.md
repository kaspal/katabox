## Enfoque: una lectura rápida y una sola pasada

**Lectura.** El registro tiene hasta 6·10⁴ líneas. Lee toda la entrada con una sola llamada y divídela por espacios; después convierte los tokens a enteros.

¿Por qué no usar `input()` una vez por línea en Python? Funciona, pero es más lento: cada llamada es un viaje completo por el intérprete. Busca `sys.stdin` y `sys.stdout`, vacía la salida, pide la línea siguiente a la capa de texto, la decodifica en un objeto cadena nuevo y le quita el salto de línea. Eso cuesta del orden de un microsegundo por línea, frente a unos pocos nanosegundos por línea con `sys.stdin.buffer.read().split()`, que hace todo el trabajo en C. Aquí son unas decenas de milisegundos. Con cientos de miles o millones de líneas, ya es una parte importante del límite de tiempo. Lo mismo pasa con un `read_line` en un `String` nuevo por línea en Rust, o con `fgets` en PHP: es correcto, pero cada llamada paga un bloqueo, una comprobación del búfer y una reserva de memoria. Leer todo de una vez es la costumbre que conviene mantener con entradas grandes.

**Cálculo.** Recorre las lecturas una sola vez y actualiza los tres números en tiempo constante por lectura:

1. `total += x`;
2. si `x > prev`, `subidas += 1`;
3. si `x == prev`, `racha += 1` y si no, `racha = 1`; después `mayor = max(mayor, racha)`.

Empieza en la primera lectura con `total = x`, `subidas = 0` y `racha = mayor = 1`.

**Por qué es correcto:** cada subida es una pareja de vecinas, que se cuenta cuando llega la segunda; una racha de lecturas iguales se alarga justo mientras la lectura siguiente es igual a la anterior, así que `mayor` ve la longitud completa de cada racha.

La suma puede llegar a 6·10⁴ · 10⁶ = 6·10¹⁰: cabe en un entero de 64 bits, y también de forma exacta en un número de JavaScript.

**Complejidad:** tiempo O(N); memoria extra O(1) además de los tokens.

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` (PHP), `read_to_string` con `split_ascii_whitespace` (Rust).
