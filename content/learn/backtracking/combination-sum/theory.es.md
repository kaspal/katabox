## La idea

Recibes un **objetivo** y unos pocos **candidatos**, y cada candidato se puede usar tantas veces como quieras. Enumera todas las combinaciones que suman exactamente el objetivo. El orden no importa: `2 + 3 + 3` y `3 + 2 + 3` son la misma combinación y deben aparecer una sola vez.

El ejemplo primero **ordena** los candidatos. Después `search(start, left)` lleva la cuenta de lo que aún falta. Si `left` es 0, se apunta el `chosen` actual. Si no, prueba cada candidato desde la posición `start`: lo elige, llama a `search(k, left − candidato)` y lo quita después. Dos detalles hacen el trabajo de verdad:

- **El índice de inicio.** La llamada pasa `k`, no `k + 1`, así que el mismo candidato se puede volver a elegir, pero nunca uno anterior. Por eso cada combinación se construye en orden no decreciente, y cada una exactamente una vez.
- **El `break`.** Cuando un candidato es mayor que lo que queda, el bucle se detiene. Eso es seguro solo porque la lista está ordenada: todos los candidatos siguientes son aún mayores. Esta **poda** corta ramas enteras sin visitarlas.

La salida es el número de combinaciones y después cada una como su tamaño seguido de sus valores. La entrada se escribe como el objetivo seguido de los candidatos.

### La entrada por defecto

Objetivo 8, candidatos `2, 3, 5`. Elegir el 2 cuatro veces llega a 0: `[2, 2, 2, 2]`. De vuelta en `2 + 2 + 2` (quedan 2), el 3 es demasiado grande: se corta. En `2 + 2`, elegir el 3 deja 1 y todo se corta; el 5 es mayor que 4. En `2`, elegir el 3 dos veces llega a 0: `[2, 3, 3]`; `2 + 5` deja 1 y se corta. Empezando por el 3, `3 + 3` deja 2 y se corta, mientras que `3 + 5` llega a 0: `[3, 5]`. Por último, el `5` solo deja 3, menos que 5, así que la búsqueda termina.

La salida es `[3, 4, 2, 2, 2, 2, 3, 2, 3, 3, 2, 3, 5]`.

## Pasos

1. Ordena los candidatos y llama a `search(0, objetivo)` con `chosen` vacío.
2. Si `left` es 0, apunta una copia de `chosen` y vuelve.
3. Para cada posición `k` desde `start`: si `candidates[k] > left`, detén el bucle; si no, añádelo, llama a `search(k, left − candidates[k])` y quítalo.
4. Devuelve el número de combinaciones y después cada una como su tamaño y sus valores.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | Depende de la salida: con una poda eficaz, el trabajo sigue al número de combinaciones encontradas |
| Caso medio | Exponencial en el objetivo |
| Peor caso | Exponencial en el objetivo: los candidatos pequeños crean árboles hondos y anchos |
| Espacio extra | `O(objetivo)` de profundidad: cada nivel resta al menos el candidato más pequeño |

## Errores comunes

- **Cortar sin ordenar.** En una lista sin ordenar, un candidato grande detendría el bucle antes de llegar a uno más pequeño, y se pierden combinaciones.
- **Volver a empezar en 0.** Entonces se enumeran `2 + 3 + 3`, `3 + 2 + 3` y `3 + 3 + 2`.
- **Pasar `k + 1`.** Entonces cada candidato solo se puede usar una vez; ese es otro problema.
- **Candidatos repetidos.** Si un valor aparece dos veces en la entrada, sus combinaciones también aparecen dos veces.

## Cuándo usarlo

Úsalo cuando hacen falta las combinaciones en sí, no solo cuántas hay o el menor número de piezas. [Cambio de monedas](learn:dynamic-programming/coin-change) cuenta o minimiza con una tabla en `O(objetivo · monedas)` y nunca enumera nada; enumerar puede producir una cantidad exponencial de respuestas. Los problemas enlazados son Combinaciones de monedas y Sellos de correo.

## Temas relacionados

- [Cambio de monedas](learn:dynamic-programming/coin-change): contar y minimizar en vez de enumerar.
- [Subconjuntos](learn:backtracking/subsets): el mismo índice de inicio, con cada valor usado como mucho una vez.
- [Corte de barras (mochila sin límite)](learn:dynamic-programming/rod-cutting): piezas reutilizables, el mejor valor con una tabla.
- [Permutaciones](learn:backtracking/permutations): cuando el orden sí importa.
