## Idea

Contar ordenaciones sería fácil, pero aquí `1 + 2` y `2 + 1` son una sola combinación.
Fija un orden para los valores de las fichas y construye cada combinación en ese orden:
`ways[a]` es el número de formas de formar `a` con los valores procesados hasta ahora,
empezando por `ways[0] = 1`. Añadir el valor `c` permite que cualquier cantidad `a ≥ c`
termine con una `c` más, así que, recorriendo las cantidades hacia arriba,
`ways[a] += ways[a − c]`. Ir hacia arriba es a propósito: reutiliza `ways[a − c]` ya
actualizado, y eso es lo que permite varias fichas del mismo valor.

El orden de los bucles importa: valores fuera y cantidades dentro cuenta combinaciones;
cantidades fuera y valores dentro contaría secuencias ordenadas.

## Complejidad

O(coins.length · amount) en tiempo, como mucho 30 · 2·10⁴ = 6·10⁵ pasos, y O(amount) de
memoria extra.
