## Enfoque: un mapa hash de unidades por artículo

Mantén `estante`, un mapa hash del código de producto a las unidades en la estantería; un código que falta significa 0 unidades. Así cada operación es una actualización de tiempo constante:

- `stock(item, qty)`: `estante[item] += qty`.
- `sell(item, qty)`: `vendidas = min(qty, estante[item])`; `estante[item] -= vendidas`; informa de `vendidas`.
- `count(item)`: informa de `estante[item]`.

Envolver el mapa en una clase pequeña con un método por operación deja la simulación legible, y el bucle principal solo elige según `ops[i]`.

### Paso a paso

Ejemplo 1: tras `stock 7 5` el estante tiene 5. `sell 7 3` vende 3 (quedan 2), `count 7` informa de 2, `sell 7 4` solo vende 2 y `count 7` informa de 0. Informes: 3, 2, 2, 0.

**Complejidad:** O(1) esperado por operación, O(ops) en total y O(artículos distintos) de memoria.
