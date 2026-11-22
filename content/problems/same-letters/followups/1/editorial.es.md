## Idea

El problema base comparaba las cuentas de letras una sola vez. Aquí guarda `diff[letra] = cuenta en a − cuenta en b` y `off`, el número de letras con `diff ≠ 0`; las bolsas son gemelas justo cuando `off == 0`. Un cambio en la posición `p` de `old` a `new` saca una `old` de `b` (`diff[old] += 1`) y mete una `new` (`diff[new] −= 1`). En cada una de las dos actualizaciones, mira si esa diferencia era cero antes y si lo es después, y corrige `off` según corresponda. Guarda `b` como un arreglo modificable para conocer `old`. Si las bolsas tienen tamaños distintos, las diferencias no pueden ser todas cero, así que la respuesta es siempre `false` sin ningún caso especial.

## Complejidad

O(|a| + |b|) para construir las cuentas y después O(1) por cambio: O(|a| + |b| + q) en tiempo en total. O(|b|) de memoria extra para la copia de `b`, más 26 contadores.
