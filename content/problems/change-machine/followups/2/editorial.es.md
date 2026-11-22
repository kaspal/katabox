## Idea

Con fichas ilimitadas, `best[a] = min(best[a − c] + 1)` sobre cada valor `c`. Con una
sola copia de cada ficha se convierte en una mochila 0/1. Procesa las fichas de una en
una, guardando `best[a]` = el menor número de las fichas vistas hasta ahora que forman
exactamente `a` (`best[0] = 0`, todo lo demás inalcanzable). Para una ficha `t`,
actualiza las cantidades desde `amount` hasta `t` con
`best[a] = min(best[a], best[a − t] + 1)`. Como el bucle va hacia abajo, `best[a − t]`
todavía describe grupos sin `t`, así que ninguna ficha se usa dos veces. Al final,
`best[amount]` es la respuesta, o `-1` si sigue siendo inalcanzable.

## Complejidad

O(tokens.length · amount) en tiempo, como mucho 100 · 10⁴ = 10⁶ pasos, y O(amount) de
memoria extra.
