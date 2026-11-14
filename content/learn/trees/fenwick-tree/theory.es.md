## La idea

Un **árbol de Fenwick** (o árbol binario indexado) responde **sumas prefijas** e **incrementos puntuales** en `O(log n)`, con un solo arreglo de `n + 1` números y unas pocas líneas de código.

Dentro del árbol, las posiciones cuentan desde 1: la casilla `i + 1` es la de la posición `i`, y la casilla 0 no se usa. Sea `lowbit(i)` el bit activo más bajo de `i`, que calcula `i & -i`: lowbit(6) = 2, lowbit(8) = 8, lowbit(5) = 1. La casilla `i` guarda la suma de las `lowbit(i)` casillas que terminan en `i`. Así, la casilla 6 cubre las casillas 5 y 6, la casilla 8 cubre de la 1 a la 8, y la casilla 5 solo a sí misma.

- **sum i** (posiciones 0 a `i`) empieza en la casilla `i + 1`, la suma y salta a `i − lowbit(i)`, la casilla justo anterior al bloque que acaba de sumar. Cada salto apaga un bit activo, así que hay como mucho `log n`.
- **add i v** empieza en la casilla `i + 1` y le suma `v`, y luego salta a `i + lowbit(i)`, la siguiente casilla cuyo bloque también contiene esta. Cada salto sube el bit activo más bajo, así que de nuevo cambian como mucho `log n` casillas.

No se dibuja ningún árbol: el patrón de bits de cada índice le dice qué bloque cubre y adónde saltar.

### La entrada por defecto

n = 8, todo a cero. add 1 4 empieza en la casilla 2 y cambia las casillas 2, 4 y 8. add 4 3 empieza en la casilla 5 y cambia 5, 6 y 8. add 6 2 empieza en la casilla 7 y cambia 7 y 8. Las casillas 1 a 8 guardan ahora 0 4 0 4 3 3 2 9. sum 5 empieza en la casilla 6 (3) y salta a la casilla 4 (4), y luego a 0, donde se para: 7. sum 7 empieza en la casilla 8, que ya cubre todas las posiciones: 9. La salida es 7, 9.

## Pasos

1. Empieza con `n + 1` ceros; la casilla 0 nunca se usa.
2. add i v: pon `i` a `i + 1`; mientras `i ≤ n`, suma `v` a la casilla `i` y pon `i` a `i + (i & -i)`.
3. sum i: pon `i` a `i + 1` y un total a 0; mientras `i > 0`, suma la casilla `i` al total y pon `i` a `i − (i & -i)`.
4. Para la suma de las posiciones `l` a `r`, calcula sum `r` menos sum `l − 1`.

## Coste

| Operación | Coste |
|---|---|
| add | `O(log n)` |
| Suma prefija | `O(log n)` |
| Espacio extra | `O(n)`: `n + 1` casillas |

## Errores comunes

- **Usar la casilla 0.** `0 & -0` es 0, así que un recorrido que llega a la casilla 0 ya no se mueve. Por eso el árbol cuenta desde 1.
- **Leer una casilla como un valor.** La casilla `i` guarda la suma de un bloque, no el valor de una posición. El valor de una posición es sum `i` menos sum `i − 1`.
- **Pedir un mínimo.** Una suma de rango sale como diferencia de dos sumas prefijas; un mínimo o un máximo de rango no se puede deshacer así. Para eso usa un árbol de segmentos.

## Cuándo usarlo

Usa un árbol de Fenwick para totales acumulados que no paran de cambiar: contar inversiones, tablas de frecuencias, la posición de un valor entre los vistos hasta ahora. Es más corto y en la práctica más rápido que un árbol de segmentos; pasa a un árbol de segmentos cuando necesites mínimos, máximos o actualizaciones de rangos.

## Temas relacionados

- [Sumas prefijas](learn:foundations/prefix-sums): las mismas respuestas cuando el arreglo no cambia.
- [Árbol de segmentos](learn:trees/segment-tree): preguntas de rango más generales, con más código.
- [Propagación perezosa](learn:trees/lazy-propagation): actualizaciones de rangos en un árbol de segmentos.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): por qué `i & -i` aísla el bit activo más bajo.
