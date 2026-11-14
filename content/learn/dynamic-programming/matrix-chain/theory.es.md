## La idea

Multiplicar una matriz `p × q` por una matriz `q × r` cuesta `p · q · r` multiplicaciones escalares y da una matriz `p × r`. Una **cadena** de matrices se puede multiplicar agrupándola de cualquier forma, porque la multiplicación de matrices es asociativa: el resultado es el mismo, pero el coste no. ¿Qué agrupación cuesta menos? La entrada es una lista `dims`, donde la matriz `k` es `dims[k] × dims[k + 1]`.

Sea cual sea la mejor agrupación de las matrices `i` a `j`, tiene una **última multiplicación**: la parte izquierda `i..k` por la parte derecha `k + 1..j`, para algún **corte** `k`. Cada parte debe multiplicarse a su vez lo más barato posible, y el producto final cuesta `dims[i] · dims[k + 1] · dims[j + 1]`. Así que:

`cost[i][j] = mínimo, entre i ≤ k < j, de cost[i][k] + cost[k + 1][j] + dims[i] · dims[k + 1] · dims[j + 1]`

Una sola matriz cuesta 0. Esto es **PD sobre intervalos**: cada celda es un tramo de la cadena y necesita las celdas de los tramos más cortos que contiene. Por eso el ejemplo rellena por **longitud del tramo**, el número de matrices que abarca: primero todos los tramos de 2, después de 3, hasta la cadena entera. En la tabla eso es una **diagonal** cada vez, avanzando hacia arriba y a la derecha, hasta acabar en la esquina superior derecha `cost[0][n - 1]`, que es la salida.

### La entrada por defecto

`dims = [8, 20, 4, 30]`: A es 8 × 20, B es 20 × 4 y C es 4 × 30. Tramos de 2: `cost[0][1] = 8 · 20 · 4 = 640` para AB, y `cost[1][2] = 20 · 4 · 30 = 2400` para BC. El tramo de 3 prueba los dos cortes. Cortando tras A, A(BC): `0 + 2400 + 8 · 20 · 30 = 7200`. Cortando tras B, (AB)C: `640 + 0 + 8 · 4 · 30 = 1600`.

La salida es `[1600]`, cuatro veces y media más barato que el otro orden.

## Pasos

1. Sea `n = len(dims) - 1`; crea una tabla `n × n` de ceros (una matriz cuesta 0).
2. Para cada longitud de tramo de 2 a `n`, y cada inicio `i` con `j = i + longitud - 1`: prueba cada corte `k` de `i` a `j - 1` y quédate con el menor total.
3. Devuelve `cost[0][n - 1]`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n³)`: unos `n²/2` tramos, cada uno probando hasta `n` cortes |
| Espacio extra | `O(n²)` para la tabla |

## Errores comunes

- **Rellenar fila a fila.** `cost[0][2]` necesita `cost[1][2]`, de una fila más abajo; ir por longitud de tramo garantiza que todos los tramos más cortos están hechos.
- **Las dimensiones equivocadas.** El último producto es `dims[i] · dims[k + 1] · dims[j + 1]`: las filas de la primera matriz, el tamaño compartido en el corte y las columnas de la última.
- **Cortes voraces.** Hacer primero el producto suelto más barato no siempre es lo mejor; solo comparar todos los cortes es seguro.

## Cuándo usarlo

Usa el patrón cuando un tramo se construye a partir de dos tramos más pequeños y el coste de unirlos depende de dónde se corta: agrupar productos, fusionar montones vecinos, cortar una pieza por varias marcas. El problema enlazado, Cortar el tronco, también es PD sobre intervalos: un trozo entre dos marcas y la elección de qué corte va primero.

## Temas relacionados

- [Corte de barras (mochila sin límite)](learn:dynamic-programming/rod-cutting): elegir cortes, con una tabla de una dimensión.
- [Mochila 0/1](learn:dynamic-programming/knapsack-01): otra tabla que se rellena en un orden cuidadoso.
- [Floyd-Warshall](learn:graphs/floyd-warshall): otra tabla `O(n³)` que prueba todos los puntos intermedios.
- [Fusionar intervalos](learn:greedy/merge-intervals): tramos tratados de forma voraz.
