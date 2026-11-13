## Enfoque: leer tokens y avanzar con una posición

La entrada mezcla cantidades y valores en muchas líneas, y algunas líneas están vacías. Leerla línea a línea obliga a tratar la forma de cada línea. Leer **todo de una vez y dividirlo por espacios** la convierte en una sola lista de tokens, donde una línea vacía no aporta nada:

```
2 3 12 30 8 1 45
```

Lleva un índice `pos` en esa lista:

1. `T = tokens[0]`, `pos = 1`.
2. Para cada día: `n = tokens[pos]`, y los precios son `tokens[pos+1 .. pos+n]`; avanza `pos` hasta después de ellos.
3. Escribe `n`, la suma y el máximo (0 cuando `n` es 0).

Construye las líneas de salida en una lista y escríbelas de una vez: una sola escritura es mucho más rápida que una por línea.

**Complejidad:** tiempo y memoria O(tamaño de la entrada).

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` (PHP), `read_to_string` con `split_ascii_whitespace` (Rust).
