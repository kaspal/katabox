## Enfoque: búsqueda binaria sobre la longitud y hashes rodantes para comprobar

**Monótono.** Un trozo repetido de longitud L contiene un trozo repetido de cada longitud menor, así que `repeats(L)` es cierto hasta la respuesta y falso después. Haz búsqueda binaria del mayor L de `[0, n − 1]` con `repeats(L)`.

**Comprobar una longitud.** Trata las letras como dígitos en base `B` y calcula los hashes prefijos `h[i]` de `text[0..i)` módulo un primo `M`, con las potencias `pw[L] = B^L mod M`. El hash de `text[i..i+L)` es `(h[i+L] − h[i] · pw[L]) mod M`. Mete los n − L + 1 hashes en un conjunto; un hash visto dos veces significa que el trozo se repite.

Trozos distintos pueden compartir hash por casualidad. Usar dos módulos independientes (y comparar el par) lo hace tan improbable que no ocurre en estas pruebas. Los módulos siguen la regla de la tanda para JavaScript: los dos son primos menores que 2²⁶ (50 331 653 y 25 165 843), así que todo producto queda por debajo de 2⁵³ y es exacto. La referencia en Python usa un solo módulo, 2⁶¹ − 1, porque sus enteros no tienen límite.

### Paso a paso

"mississippi": L = 5 no tiene repetición, L = 4 sí ("issi" en 1 y 4), así que la respuesta es 4.

**Complejidad:** O(n log n) en tiempo esperado y O(n) de espacio. Comparar cada par de posiciones de inicio letra a letra es O(n²) pares como mínimo.
