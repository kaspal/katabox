## Idea

Mira los estantes a través de un array de diferencias `d`: una entrega `[a, b, count]` suma
`count` a `d[a]` y lo resta de `d[b + 1]`, y el valor de un estante es la suma de `d` hasta
él. El total de los primeros `i` estantes es entonces
`Σ_{j<i} (i − j) · d[j] = i · Σ d[j] − Σ j · d[j]`. Guarda un árbol de Fenwick para `d[j]`
y otro para `j · d[j]`; cada entrega son dos actualizaciones puntuales en cada árbol, y
cada total prefijo son dos consultas prefijas. Carga el stock inicial como una entrega de
`stock[i]` al estante `i` solo, y responde `[a, b]` como `prefix(b + 1) − prefix(a)`. (Un
árbol de segmentos con sumas perezosas también sirve.)

## Complejidad

O((n + q) log n) en tiempo para `n` estantes y `q` operaciones, y O(n) de memoria extra.
Los totales pueden llegar a unos 10¹⁴: usa enteros de 64 bits; en JavaScript, los números
son exactos hasta 2⁵³.
