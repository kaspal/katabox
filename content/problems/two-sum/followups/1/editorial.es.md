## Idea

El problema base se detenía en el primer par; aquí cuentan todos. Recorre de izquierda a
derecha con un mapa `seen` de valor a cuántas veces ha aparecido. Al llegar a un número
`x`, cada `target − x` anterior forma un par con él: suma `seen[target − x]` a la
respuesta y después registra `x`. Contar antes de registrar mantiene `i < j` y nunca
empareja una posición consigo misma.

## Complejidad

O(n) en tiempo y O(n) de memoria extra. La respuesta puede llegar a unos 5·10⁹: usa un
entero de 64 bits (un número de JavaScript es exacto hasta 2⁵³).
