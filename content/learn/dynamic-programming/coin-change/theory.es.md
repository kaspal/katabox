## La idea

Paga una **cantidad** exacta con el **menor número de monedas**, con una reserva ilimitada de cada valor. El plan obvio es voraz: toma la moneda más grande que quepa y repite. Funciona con muchas monedas reales, pero no con cualquier conjunto de monedas.

La programación dinámica hace una pregunta más pequeña para cada total desde 0 hasta la cantidad: ¿cuál es el menor número de monedas que suman **exactamente este total**? Llamémoslo `fewest[t]`. Sea cual sea la mejor forma de pagar `t`, tiene una **última moneda** `c`, y lo que va antes es una mejor forma de pagar `t - c`. Así que:

`fewest[t] = mínimo, entre las monedas c ≤ t, de fewest[t - c] + 1`

`fewest[0] = 0`: con ninguna moneda se paga nada. Las demás celdas empiezan en 999, el valor que el ejemplo usa para "todavía no se puede formar" (mayor que cualquier cuenta real aquí). El ejemplo prueba las monedas en el orden dado y **mejora** una celda solo cuando `fewest[t - c] + 1` es estrictamente menor. Si al final `fewest[cantidad]` sigue en 999, ninguna combinación sirve y la salida es `[-1]`; si no, es `[fewest[cantidad]]`.

### La entrada por defecto

Cantidad 12, monedas 1, 6 y 9 (el ajuste "el voraz falla"). El voraz toma 9 y quedan 3, que solo se pagan con unos: 9 + 1 + 1 + 1, **cuatro monedas**.

La tabla: los totales 1 a 5 solo pueden usar unos, así que reciben de 1 a 5. El total 6 recibe primero 6 con la moneda 1 y después la moneda 6 lo mejora a 1. Los totales 7 y 8 reciben 2 y 3. El total 9 baja a 1 con la moneda 9. Los totales 10 y 11 reciben 2 y 3. Para 12, la moneda 1 ofrece `fewest[11] + 1 = 4`, la moneda 6 ofrece `fewest[6] + 1 = 2` y la moneda 9 ofrece `fewest[3] + 1 = 4`. La tabla es `[0, 1, 2, 3, 4, 5, 1, 2, 3, 1, 2, 3, 2]`.

La salida es `[2]`: 6 + 6. El voraz se comprometió con el 9 y nunca miró atrás; la tabla compara todas las últimas monedas posibles.

## Pasos

1. Pon `fewest[0] = 0` y todos los demás totales a 999.
2. Para cada total `t` de 1 a la cantidad, y cada moneda `c ≤ t`: si `fewest[t - c] + 1 < fewest[t]`, pon `fewest[t]` a ese valor.
3. Devuelve `-1` si `fewest[cantidad]` sigue en 999 y, si no, `fewest[cantidad]`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(cantidad · monedas)`: cada total prueba cada moneda una vez |
| Espacio extra | `O(cantidad)` para la tabla |

## Errores comunes

- **Fiarse del voraz.** Solo es seguro con conjuntos de monedas que se sabe que lo admiten; basta un contraejemplo como 1, 6, 9 para romperlo.
- **Sumar 1 a "imposible".** Con un centinela como 999, `999 + 1` sigue siendo demasiado grande para ganar, así que aquí no hace daño; con el entero máximo real se desbordaría en muchos lenguajes.
- **Leer por debajo de 0.** Comprueba `c ≤ t` antes de leer `fewest[t - c]`.
- **Olvidar el -1.** Con el ajuste "no se puede" (cantidad 7, monedas 2 y 4) todos los totales impares se quedan en 999.

## Cuándo usarlo

Úsalo para dar cambio exacto, alcanzar un objetivo con pasos de tamaño fijo o cualquier pregunta del tipo "el menor número de piezas que suman exactamente `t`". El problema enlazado, Máquina de cambio, es justo esto.

## Temas relacionados

- [Corte de barras (mochila sin límite)](learn:dynamic-programming/rod-cutting): la misma reutilización de piezas, maximizando un precio.
- [Mochila 0/1](learn:dynamic-programming/knapsack-01): cuando cada objeto solo se puede usar una vez.
- [Búsqueda en anchura (BFS)](learn:graphs/bfs): los totales como nodos y las monedas como aristas; el mínimo de monedas es el mínimo de aristas desde 0.
- Algoritmos voraces (próximamente): cuándo la elección voraz es segura y cómo demostrarlo.
