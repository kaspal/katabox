## Enfoque: ordenar cada sección y tomar el medio inferior

**Lectura.** La entrada contiene muchas secciones, cada una con una cantidad seguida de ese número de alturas. Lee **todo de una vez y divídelo por espacios**, y recorre los tokens con una posición `pos`: `T = tokens[0]`; para cada sección, `n = tokens[pos]` y las alturas son los `n` tokens siguientes; avanza `pos` hasta después de ellos.

**La mediana.** Ordena las alturas de la sección. Con posiciones desde 0, el cantante del medio está en

```
(n − 1) / 2     (división entera)
```

- `n = 5`: posición 2, el tercero de cinco, con dos cantantes a cada lado.
- `n = 4`: posición 1. Los dos del medio están en las posiciones 1 y 2, y el más bajo va primero en el orden.
- `n = 1`: posición 0, el único cantante.

**Por qué es correcto:** tras ordenar, exactamente `(n − 1) / 2` cantantes quedan antes de la posición `(n − 1) / 2`. Con `n` impar quedan los mismos después, así que es el del medio. Con `n` par queda uno más después que antes, así que es el primero de los dos del medio, y en el orden el primero es el más bajo. Las alturas repetidas no cambian nada: los valores iguales quedan juntos, y el que está en esa posición sigue siendo la respuesta.

**Complejidad:** O(n log n) por sección para ordenar, O(Σn log n) en total, con Σn ≤ 10⁵.

**Trampas de cada lenguaje:**

- En TypeScript, `Array.prototype.sort()` sin argumento compara **como cadenas**, así que `[9, 10]` queda como `[10, 9]`. Convierte los tokens con `Number` y pasa `(a, b) => a - b`, o usa un `Float64Array`/`Int32Array`, cuyo `sort()` es numérico. Usa `Math.floor((n - 1) / 2)` o `(n - 1) >> 1` para la división entera.
- PHP: convierte los tokens a `int` antes de `sort()`; `intdiv($n - 1, 2)` da la posición.
- Rust: `sort_unstable()` sobre un `Vec<u32>` o `Vec<i64>`.

**Entrada y salida rápidas:** lee toda la entrada estándar de una vez, reúne las `T` respuestas y escríbelas de una sola vez.
