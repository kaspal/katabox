## Enfoque: precalcular la respuesta de cada prefijo con un conjunto

Contar desde cero los identificadores distintos de `escaneos[0 .. p]` cuesta O(p) por pregunta, es decir O(N·Q) en total: cientos de millones de inserciones en un conjunto en los límites. Pero todas las preguntas son sobre un **prefijo** de la misma lista, y cada prefijo es el anterior más un escaneo.

Recorre los escaneos una vez, con un conjunto `vistos` y un arreglo `distintos` con `distintos[0] = 0`:

```
para p en 1..N:
    añade escaneos[p-1] a vistos
    distintos[p] = tamaño de vistos
```

El escaneo `p` hace crecer el conjunto en uno justo cuando su identificador es nuevo, así que `distintos[p]` es el número de identificadores distintos entre los primeros `p` escaneos. Cada pregunta se responde entonces con `distintos[p]` en O(1), lleguen las preguntas en el orden que lleguen.

Lee todos los tokens de una vez: los identificadores están en una línea y las preguntas en muchas, pero al dividir por espacios forman una sola lista (`N`, los identificadores, `Q`, los valores de `p`). Junta las respuestas y escríbelas de una vez.

**Complejidad:** tiempo esperado O(N + Q), memoria O(N).

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` (PHP), `read_to_string` con `split_ascii_whitespace` (Rust).
