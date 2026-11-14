## La idea

Una escalera tiene `n` escalones, y cada movimiento sube **1 escalón o 2**. ¿En cuántos órdenes distintos se puede llegar arriba? Enumerar todos los órdenes sirve para escaleras diminutas, pero la cuenta crece cerca de un 60 % con cada escalón más.

Mira en cambio el **último movimiento**. Toda forma de llegar al escalón `i` termina o bien con un movimiento de 1 desde el escalón `i - 1`, o bien con uno de 2 desde el escalón `i - 2`. Ninguna termina de las dos maneras y todas terminan de una de ellas, así que los dos grupos simplemente se suman:

`ways[i] = ways[i - 1] + ways[i - 2]`

La pregunta para `i` se responde con las respuestas de escaleras más pequeñas, que es la subestructura óptima; cada una de esas respuestas se usa dos veces, que son los subproblemas superpuestos. Una tabla llenada de abajo arriba resuelve ambas cosas.

El caso base es `ways[0] = 1`: hay exactamente una forma de estar abajo, no moverse. Gracias a él, `ways[2] = 2` sale bien (1 + 1, o 2). El ejemplo llena cada celda en dos movimientos: copia `ways[i - 1]` (el último movimiento fue de 1) y después, cuando `i ≥ 2`, suma `ways[i - 2]` (el último movimiento fue de 2). Devuelve la tabla completa `ways[0..n]`.

Es otra vez la regla de Fibonacci, desplazada en uno: `ways[i] = fib(i + 1)`.

### La entrada por defecto

`n = 6`. La celda 1 copia `ways[0]`, así que `ways[1] = 1`: un solo paso. La celda 2 copia 1 y suma 1, lo que da 2. La celda 3 recibe 2 + 1 = 3: los órdenes 1 + 1 + 1, 1 + 2 y 2 + 1. La celda 4 recibe 3 + 2 = 5, la celda 5 recibe 5 + 3 = 8 y la celda 6 recibe 8 + 5 = 13.

La salida es `[1, 1, 2, 3, 5, 8, 13]`: 13 formas de subir seis escalones.

## Pasos

1. Crea una tabla de `n + 1` celdas y pon `ways[0] = 1`.
2. Para `i` de 1 a `n`, pon `ways[i] = ways[i - 1]`.
3. Si `i ≥ 2`, súmale `ways[i - 2]`.
4. Devuelve la tabla; `ways[n]` es la respuesta.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n)`: una celda por escalón, cada una a partir de dos celdas anteriores |
| Espacio extra | `O(n)` para la tabla, u `O(1)` con dos variables para las dos últimas celdas |

## Errores comunes

- **`ways[0] = 0`.** Parece natural ("cero escalones, cero movimientos"), pero entonces cada celda suma ceros y toda la tabla se queda en 0.
- **Desfase de uno.** Los escalones de 0 a `n` necesitan `n + 1` celdas.
- **Contar conjuntos en vez de órdenes.** Aquí 1 + 2 y 2 + 1 son formas distintas. Contar qué tamaños de movimiento se usan, sin importar el orden, necesita otro bucle.
- **Cuentas enormes.** Los números crecen unas 1,6 veces por escalón; los problemas reales suelen pedir la respuesta módulo un primo grande, aplicado tras cada suma.

## Cuándo usarlo

El patrón "contar las formas de construir una longitud `n` con piezas" sirve para embaldosados, códigos hechos de símbolos cortos y caminos que avanzan a saltos fijos. El problema enlazado, Baldosas del sendero, añade un **tercer tamaño de baldosa**. El argumento del último movimiento sigue valiendo; averiguar qué cambia en la fórmula es el ejercicio.

## Temas relacionados

- [Fibonacci: memo y tabla](learn:dynamic-programming/fibonacci-memo-table): la misma regla y las dos formas de calcularla.
- [El ladrón de casas](learn:dynamic-programming/house-robber): otra tabla que mira dos celdas atrás.
- [Cambio de monedas](learn:dynamic-programming/coin-change): piezas de varios tamaños, contando el mínimo en vez de las formas.
- [Caminos por una cuadrícula](learn:dynamic-programming/grid-paths): contar rutas en dos dimensiones.
