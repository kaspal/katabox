## La idea

`C(n, k)`, que se lee "n sobre k", cuenta las formas de elegir `k` elementos de entre `n` cuando el orden no importa. La fórmula `n! / (k! · (n - k)!)` es correcta, pero los factoriales se disparan mucho antes que la respuesta: `20!` ya necesita 19 cifras, mientras que `C(20, 10)` es solo 184 756.

El **triángulo de Pascal** construye los mismos números solo con sumas. Fíjate en el último de los `n` elementos. Toda elección de `k` elementos o bien **lo deja fuera**, y entonces elige los `k` entre los otros `n - 1`, o bien **lo toma**, y entonces elige `k - 1` más entre los otros `n - 1`. Ninguna elección está en los dos grupos, así que:

`C(n, k) = C(n - 1, k - 1) + C(n - 1, k)`

Los bordes valen 1: hay una forma de no elegir nada y una de elegirlo todo. Así que cada fila empieza y termina en 1, y cada valor interior es **la suma de los dos que tiene encima**. La fila `n` enumera de `C(n, 0)` a `C(n, n)`.

Es una **tabla de programación dinámica**: cada celda es una pregunta más pequeña, respondida una vez y reutilizada por la fila de abajo. El ejemplo construye las filas 0 a `n` y devuelve la fila `n`. La entrada es un único `n` de 1 a 12.

### La entrada por defecto

`n = 6`. Las filas crecen así:

`1` · `1 1` · `1 2 1` · `1 3 3 1` · `1 4 6 4 1` · `1 5 10 10 5 1` · `1 6 15 20 15 6 1`

Por ejemplo, `C(6, 2) = C(5, 1) + C(5, 2) = 5 + 10 = 15`, y el central `C(6, 3) = 10 + 10 = 20`. La salida es `[1, 6, 15, 20, 15, 6, 1]`, tras 15 sumas (las filas 2 a 6 tienen 1, 2, 3, 4 y 5 celdas interiores).

La fila suma `64 = 2⁶`. No es casualidad: todo subconjunto de 6 elementos tiene algún tamaño `k`, y hay `2⁶` subconjuntos, porque cada elemento está dentro o fuera. Cada fila duplica la suma de la anterior, porque cada valor de arriba se suma en dos celdas de abajo.

## Pasos

1. Empieza con la fila 0: `[1]`.
2. Para `i` de 1 a `n`: crea una fila de `i + 1` unos; para `k` de 1 a `i - 1`, haz `row[k] = above[k - 1] + above[k]`.
3. Devuelve la fila `n`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n²)`: unas `n² / 2` sumas |
| Espacio extra | `O(n²)` para todo el triángulo, u `O(n)` guardando una sola fila |

Para guardar una sola fila, actualízala **de derecha a izquierda**: así `row[k] += row[k - 1]` todavía lee el `row[k - 1]` antiguo.

## Errores comunes

- **Factoriales primero.** `n!` desborda los enteros de 64 bits en `n = 21`, aunque `C(n, k)` sea pequeño; la tabla nunca construye un número mayor que la respuesta.
- **De izquierda a derecha en una sola fila.** Lee celdas ya actualizadas en esta fila y produce valores erróneos.
- **Olvidar `k > n`.** Hay 0 formas de elegir más elementos de los que existen; compruébalo antes de leer la tabla.

## Cuándo usarlo

Usa la tabla cuando `n` llegue como mucho a unos pocos miles, o cuando necesites muchos `C(n, k)` a la vez, quizá módulo un número, porque solo suma. Los caminos en una cuadrícula son un caso clásico: un paseo de `r` pasos al sur y `c` al este es elegir cuáles de los `r + c` pasos van al sur. Los problemas enlazados, Elegir equipos y Paseos con cortes, llegan a `n = 10⁵`, demasiado para una tabla; ahí toman el relevo los factoriales módulo un primo, divididos con un [Inverso modular](learn:math-geometry/modular-inverse).

## Temas relacionados

- [Caminos por una cuadrícula](learn:dynamic-programming/grid-paths): la misma regla de suma, celda a celda.
- [Subir escaleras](learn:dynamic-programming/climbing-stairs): otra tabla donde cada valor es la suma de valores anteriores.
- [Inverso modular](learn:math-geometry/modular-inverse): el camino con divisiones para `n` grandes.
- [Subconjuntos](learn:backtracking/subsets): enumerar las `2ⁿ` elecciones una a una en lugar de contarlas.
