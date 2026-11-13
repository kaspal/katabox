## Enfoque: extraer las palabras, contarlas y ordenar los recuentos

**Leer texto libre.** Solo la primera línea tiene un significado fijo. Después viene texto de longitud desconocida, así que no hay ninguna cantidad con la que hacer un bucle: lee toda la entrada de una vez, toma la primera línea como `K` y trata el resto como una sola cadena. Las palabras no se separan solo con espacios (`rain,rain!sun`), así que dividir por espacios dejaría signos pegados a las palabras. Mejor extrae las palabras directamente: cada secuencia máxima de `a`-`z`, que la expresión regular `[a-z]+` encuentra en una sola llamada. Los saltos de línea y las líneas vacías no necesitan ningún cuidado especial.

**Recuento.** Un mapa hash de palabra a cantidad, que se actualiza una vez por palabra.

**Clasificación.** Ordena las palabras distintas por cantidad descendente y, a igual cantidad, alfabéticamente; después escribe las `K` primeras. El `sorted` de Python con la clave `(-cantidad, palabra)` expresa las dos reglas; en otros lenguajes compara primero las cantidades y después las palabras. Compara las palabras por sus caracteres (`<` entre cadenas en TypeScript, `strcmp` en PHP, `Ord` de `&str` en Rust), no con una comparación que dependa del idioma.

**Por qué es correcto:** cada palabra se cuenta exactamente una vez y el orden es exactamente el pedido, así que las `K` primeras entradas son la respuesta (o todas, si hay menos de `K`).

**Complejidad:** tiempo O(L + D log D), donde L es la longitud del texto y D el número de palabras distintas; memoria O(L).

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read()` con `re.findall(rb"[a-z]+", ...)` y un `Counter` (Python), `readFileSync(0, "utf8")` con `match(/[a-z]+/g)` y un `Map` (TypeScript), `stream_get_contents(STDIN)` con `preg_match_all` y `array_count_values` (PHP), `read_to_string`, `split(|c: char| !c.is_ascii_lowercase())` y un `HashMap` (Rust).
