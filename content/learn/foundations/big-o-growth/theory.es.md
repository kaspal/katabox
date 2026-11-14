## La idea

La notación Big-O describe cómo **crece** el trabajo de un algoritmo cuando crece su entrada, sin fijarse en los factores constantes. La animación ejecuta seis bucles pequeños con el mismo `n` y cuenta sus pasos (`steps`): un paso, un bucle que dobla, un bucle sobre `n`, un bucle que dobla dentro de un bucle sobre `n`, dos bucles anidados y un bucle sobre todos los subconjuntos de `n` elementos. Los recuentos empiezan cerca y se separan muy deprisa; la gráfica muestra cada recuento como una curva en función de `n`.

## Pasos

1. `constant`: un paso, valga lo que valga `n`.
2. `logarithmic`: `i` empieza en 1 y se dobla mientras `i <= n`, así que llega a `n` en unos `log₂ n` pasos.
3. `linear`: un paso por valor, `n` pasos.
4. `linearithmic`: el bucle que dobla se ejecuta una vez por cada uno de los `n` valores: unos `n log n` pasos.
5. `quadratic`: un paso por cada par `(i, j)`: `n²` pasos.
6. `exponential`: un paso por cada uno de los `2ⁿ` subconjuntos.
7. Mueve el control de `n` y mira cómo se asienta el orden de las curvas.

## Coste

| Crecimiento | Pasos con n = 10 | Pasos con n = 16 |
|---|---|---|
| `O(1)` | 1 | 1 |
| `O(log n)` | 4 | 5 |
| `O(n)` | 10 | 16 |
| `O(n log n)` | 40 | 80 |
| `O(n²)` | 100 | 256 |
| `O(2ⁿ)` | 1 024 | 65 536 |

## Errores comunes

- **Fiarse de un `n` pequeño.** Con `n = 3` el bucle cuadrático cuenta 9 pasos y el exponencial solo 8. Big-O habla de lo que pasa cuando `n` crece: a partir de `n = 5`, `2ⁿ` va siempre por delante.
- **Multiplicar por `n` cada bucle anidado.** El bucle interno de `linearithmic` dobla `j`, así que suma `log n` pasos por vuelta, no `n`. Fíjate en cómo cambia cada variable de bucle.
- **Meter constantes en Big-O.** `2n` y `n + 100` son ambos `O(n)`, y la base del logaritmo da igual. Las constantes sí cuentan al comparar dos algoritmos de la misma clase.

## Cuándo usarlo

Usa Big-O antes de escribir código: estima `n` a partir de los límites del problema y elige un algoritmo cuyo crecimiento quepa. Como guía aproximada, un ordenador hace unos 10⁸ pasos sencillos por segundo, así que `n²` va bien con `n` de unos miles, `n log n` con millones y `2ⁿ` solo hasta unos 25.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): de dónde sale una profundidad `log n`.
- [Coste amortizado: el arreglo dinámico](learn:foundations/dynamic-array): el coste promediado sobre muchas operaciones.
- [Ordenación rápida](learn:sorting/quick-sort): `O(n log n)` en promedio, `O(n²)` en el peor caso.
- [Ordenación de burbuja](learn:sorting/bubble-sort): un algoritmo `O(n²)` sin más.
