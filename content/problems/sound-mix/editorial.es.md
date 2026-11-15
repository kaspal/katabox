## Enfoque: como mucho K menos como mucho K − 1, con una ventana deslizante

Contar directamente los tramos con **exactamente** `K` fuentes es incómodo: al crecer, un tramo puede pasar por `K` y seguir de largo. Contar los tramos con **como mucho** `m` fuentes es fácil, y

```
exactamente K  =  (como mucho K)  −  (como mucho K − 1)
```

porque un tramo con como mucho `K` fuentes tiene o bien exactamente `K`, o bien como mucho `K − 1`.

**Como mucho m, con una ventana.** Si un tramo tiene como mucho `m` fuentes distintas, también las tiene cualquier tramo dentro de él. Así que, para un final fijo `right`, los inicios válidos son `left..right` para un inicio mínimo `left`, y cuando `right` avanza, `left` solo puede quedarse o avanzar. Guarda un mapa hash de cada id a su contador dentro de la ventana:

1. Añade `ids[right]` al mapa.
2. Mientras el mapa tenga más de `m` ids, saca `ids[left]` (borra el id cuando su contador llegue a 0) y avanza `left`.
3. Todos los tramos que empiezan en `left..right` y acaban en `right` valen: suma `right − left + 1`.

Con `m = 0` la respuesta es 0 (ningún tramo está vacío), lo que cubre el caso `K = 1`.

**Ejemplo** (`4 4 7 4 9 9`, `K = 2`). Como mucho 2: la ventana suma 1, 2, 3, 4; luego `9` trae una tercera fuente, la ventana se reduce a `4 9` y suma 2, y después 3: 15 en total. Como mucho 1: 1, 2, 1, 1, 1, 2: 8. La respuesta es 15 − 8 = 7.

**Por qué es correcto:** el paso 2 quita justo los inicios que dejarían más de `m` fuentes, y ningún inicio anterior a `left` puede volver a ser válido más adelante, porque un tramo más largo nunca tiene menos fuentes. Así, el paso 3 cuenta cada tramo con como mucho `m` fuentes una sola vez, en su final.

**Complejidad:** cada segundo entra y sale de la ventana como mucho una vez por pasada, así que el tiempo esperado es O(n) con un mapa hash, y la memoria O(n).

**Por qué lo obvio es demasiado lento:** extender desde cada inicio hasta tener más de `K` fuentes es O(n²). Con `n = 10⁵` y una grabación en la que la mayoría de los tramos se quedan en `K` fuentes hasta el final, son unos 5·10⁹ pasos.

**Notas prácticas:** lee todos los tokens de una vez y escribe la única respuesta. Los ids llegan a 10⁹, así que cuéntalos en un mapa hash (`dict`, `Map`, un arreglo de PHP con claves enteras, `HashMap`), no en un arreglo indexado por id. El recuento puede llegar a `n(n + 1)/2 ≈ 5·10⁹`, que desborda 32 bits: usa `u64` o `i64` en Rust (en un `Number` de TypeScript sigue siendo exacto).
