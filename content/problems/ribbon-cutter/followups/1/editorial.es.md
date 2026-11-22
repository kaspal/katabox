## Idea

El problema base hacía búsqueda binaria sobre la respuesta con una comprobación O(n);
repetirla para 10⁵ pedidos es demasiado lento. En su lugar, calcula `count[L]`, el
número de trozos de longitud `L`, para cada `L` de 1 a `m = max(ribbons)`. Construye
`atMost[v]`, cuántos rollos miden como mucho `v`; un rollo da un trozo por cada múltiplo
de `L` que alcanza, así que `count[L]` es la suma de `n − atMost[k·L − 1]` sobre los
múltiplos `k·L ≤ m`. Como `count` nunca crece, la respuesta a un pedido `need` es cuántas
longitudes cumplen `count[L] ≥ need`, y eso sale con una búsqueda binaria (0 si ni
`count[1]` alcanza).

## Complejidad

O(m log m) en tiempo para llenar `count` (la suma armónica m/1 + m/2 + … + m/m) y
después O(log m) por pedido; O(m) de memoria extra. El número de trozos llega a 10¹⁰:
guárdalo y compáralo con enteros de 64 bits.
