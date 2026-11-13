## Enfoque: dos búsquedas en anchura

La ruta más larga entre dos cruces de un árbol es su **diámetro**. Una búsqueda en anchura (BFS) desde un cruce da el número de senderos hasta todos los demás en O(N), así que lanzar una desde cada cruce y quedarse con la mayor distancia es correcto, pero cuesta O(N²) en total.

Bastan dos búsquedas:

1. BFS desde cualquier cruce `s` (por ejemplo, el 1); sea `u` un cruce a la mayor distancia.
2. BFS desde `u`; la mayor distancia encontrada es el diámetro.

**Por qué funciona.** En un árbol, el cruce más lejano desde cualquier punto de partida siempre es un extremo de alguna ruta más larga. Idea: toma una ruta más larga `x … y`; la ruta de `s` a `u` la toca en algún cruce `p` (o pasa lo más cerca de ella en `p`). Como ningún cruce está más lejos de `s` que `u`, `u` está al menos tan lejos de `p` como `x` y como `y`, así que cambiar `x` (o `y`) por `u` da una ruta al menos igual de larga, con `u` como extremo. La segunda BFS mide la ruta más larga desde `u`, así que encuentra el diámetro.

**Implementación:** lee todos los tokens de una vez; para cada parque construye las listas de adyacencia (o un arreglo compacto de vecinos con desplazamientos) a partir de las `N − 1` aristas. Usa una BFS **iterativa** con un arreglo como cola: un parque puede ser una sola cadena de 20 000 cruces, demasiado profunda para la recursión en la mayoría de los lenguajes. Un parque con `N = 1` no tiene líneas de aristas y su respuesta es 0.

**Complejidad:** tiempo y memoria O(N) por parque, O(suma de los N) en total.

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` con arreglos tipados (TypeScript), `stream_get_contents(STDIN)` con `preg_split` (PHP), `read_to_string` con `split_ascii_whitespace` (Rust).
