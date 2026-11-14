## La idea

Dada una fila de números, algunos negativos, encuentra el **tramo contiguo** con la mayor suma. Probar todos los inicios y finales cuesta `O(n²)` incluso con sumas acumuladas. El **algoritmo de Kadane** lo hace en una sola pasada.

Es programación dinámica con una pregunta bien elegida: ¿cuál es la mejor suma de un tramo que **termina exactamente en el índice `i`**? Ese tramo o bien es el valor solo, o bien alarga el mejor tramo que termina en `i - 1`. Alargarlo solo ayuda si esa suma anterior es positiva, así que:

`current(i) = valor + current(i - 1)` si `current(i - 1) > 0`, y si no `valor`

La respuesta es el mayor `current(i)` de todos los `i`. Cada paso necesita solo la celda anterior, así que la "tabla" se reduce a una variable, `current`, más el inicio de su tramo.

El ejemplo **reinicia** el tramo en `i` cuando `current ≤ 0` y lo **alarga** en otro caso. Tras cada paso **apunta** el tramo si su suma es estrictamente mejor que la mejor hasta ahora. La mejor empieza en el primer valor, no en 0, así que una fila solo de negativos también funciona: la respuesta es el mayor valor suelto. La salida es `[mejor, inicio, fin]`, con índices que empiezan en 0.

### La entrada por defecto

`-2, 3, -1, 4, -6, 2, 5, -3, 1`. El índice 0 empieza un tramo en -2, que empata con la mejor inicial y no se vuelve a apuntar. La suma es negativa, así que el índice 1 reinicia en 3: se apunta, `[3, 1, 1]`. El índice 2 alarga a 2. El índice 3 alarga a 6: se apunta, `[6, 1, 3]`. El índice 4 alarga a 0. Como 0 no es positivo, el índice 5 reinicia en 2 con inicio 5. El índice 6 alarga a 7: se apunta, `[7, 5, 6]`. Los índices 7 y 8 dan 4 y 5, no mejoran.

La salida es `[7, 5, 6]`: `2 + 5`. El tramo de 1 a 6 también suma 7, pero reiniciar con una suma de 0 descarta un prefijo que no aporta nada, y "estrictamente mejor" conserva el primer tramo encontrado.

Con el ajuste de todos negativos `-4, -2, -7, -1, -5`, todos los índices reinician y la salida es `[-1, 3, 3]`.

## Pasos

1. Pon `best` al primer valor con inicio y fin 0, y `current` a 0.
2. Para cada índice `i`: si `current ≤ 0`, pon `current` al valor y el inicio del tramo a `i`; si no, suma el valor a `current`.
3. Si `current > best`, apunta `current`, el inicio del tramo e `i`.
4. Devuelve `[best, inicio, fin]`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n)`: una pasada, trabajo constante por valor |
| Espacio extra | `O(1)`: el tramo actual y el mejor |

## Errores comunes

- **Empezar `best` en 0.** Con una fila toda negativa daría 0, la suma de un tramo vacío, que aquí no se permite.
- **Reiniciar solo cuando `current < 0`.** La mejor suma es la misma, pero los tramos pueden arrastrar un prefijo inútil que suma 0, así que cambian los índices.
- **Olvidar el inicio.** Apunta el inicio junto con la suma, o no se podrá dar el tramo.
- **Apuntar con `≥`.** Eso lleva la respuesta a un tramo posterior en caso de empate.

## Cuándo usarlo

Usa Kadane para el mejor periodo de ganancias diarias, el segmento más fuerte de una señal, o como pieza de otro algoritmo, por ejemplo el mejor rectángulo de una cuadrícula fijando dos filas y aplicándolo a las sumas de las columnas.

## Temas relacionados

- [Sumas prefijas](learn:foundations/prefix-sums): la suma de un tramo como diferencia de dos prefijos.
- [Ventana deslizante](learn:searching/sliding-window): tramos con un tamaño o una condición en vez de una mejor suma.
- [El ladrón de casas](learn:dynamic-programming/house-robber): otra decisión en una pasada sobre una fila de valores.
- [Subsecuencia creciente más larga](learn:dynamic-programming/lis): la mejor respuesta que termina en cada índice, mirando más atrás.
