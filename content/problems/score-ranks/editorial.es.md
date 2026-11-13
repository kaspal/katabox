## Enfoque: ordenar, quitar repetidos y buscar

Para una puntuación `x` que alguien sacó, su puesto es **1 + el número de puntuaciones distintas mayores que `x`**. Calcularlo en cada pregunta recorriendo las `N` puntuaciones (y juntando las mayores en un conjunto) cuesta O(N) por pregunta, O(N·Q) en total: cientos de millones de pasos en los límites.

Mejor hacer el trabajo una sola vez:

1. Toma las puntuaciones distintas y ordénalas de mayor a menor: `100 85 70 60` en el ejemplo 1.
2. La puntuación en el índice `j` (desde 0) de esa lista tiene exactamente `j` puntuaciones distintas por encima, así que su puesto es `j + 1`.
3. Para cada puntuación preguntada, búscala en la lista. Puedes usar búsqueda binaria (la lista está ordenada) o construir una vez un mapa `puntuación → puesto` y consultarlo. Si no está, nadie sacó esa puntuación: escribe `0`.

Las puntuaciones iguales comparten puesto porque los repetidos se quitaron antes de numerar: en el ejemplo 1 los dos 85 tienen el puesto 2, y el 70 tiene el puesto 3, no el 4.

Lee todos los tokens de una vez (`N`, las puntuaciones, `Q`, las puntuaciones preguntadas) y escribe las respuestas unidas por espacios en una sola línea.

**Complejidad:** tiempo O(N log N + Q log N) con búsqueda binaria (O(N log N + Q) esperado con un mapa), memoria O(N).

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` (PHP), `read_to_string` con `split_ascii_whitespace` (Rust).
