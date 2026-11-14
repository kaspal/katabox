## La idea

¿Comparten algún punto dos segmentos, `p1p2` y `p3p4`? Resolver las ecuaciones de las dos rectas funciona, pero necesita divisiones, fracciones y un caso especial para las rectas paralelas. La prueba de orientación responde solo con multiplicaciones de números enteros.

`orient(a, b, c)` es el signo del **producto cruzado** `(b - a) × (c - a)`: `+1` cuando `c` queda a la izquierda de la dirección `a → b`, `-1` cuando queda a la derecha y `0` cuando los tres puntos están en una misma recta. El ejemplo hace cuatro de estas pruebas:

- `d1`, `d2`: los lados de `p1` y `p2` respecto al segmento `p3p4`;
- `d3`, `d4`: los lados de `p3` y `p4` respecto al segmento `p1p2`.

Hay dos formas de encontrarse:

- **Un cruce propio.** `p1` y `p2` quedan a lados opuestos de la recta que pasa por `p3p4` (`d1 · d2 < 0`), **y** `p3` y `p4` quedan a lados opuestos de la recta que pasa por `p1p2` (`d3 · d4 < 0`). Una sola condición no basta: un segmento corto puede quedar a ambos lados de una recta larga muy lejos de donde termina el otro segmento.
- **Un contacto.** Alguna prueba da 0, así que ese punto está en la recta del otro segmento; está de verdad en el segmento cuando además queda dentro de su **caja envolvente** (entre sus `x` e `y` mínimas y máximas).

La salida es `[1 o 0, d1, d2, d3, d4]`: 1 cuando los segmentos se tocan. Los dos segmentos se escriben como cuatro puntos `x y`, con coordenadas de 0 a 20.

### La entrada por defecto

`p1 = (2, 3)`, `p2 = (15, 14)`, `p3 = (3, 16)`, `p4 = (17, 2)` (el ajuste "se cruzan"). Visto desde `p3 → p4`, `p1` queda a la derecha (`d1 = -1`) y `p2` a la izquierda (`d2 = 1`). Visto desde `p1 → p2`, `p3` queda a la izquierda (`d3 = 1`) y `p4` a la derecha (`d4 = -1`). Los dos pares quedan a ambos lados, así que la salida es `[1, -1, 1, 1, -1]`.

"Un extremo sobre la otra recta" (`(3, 3)`, `(15, 9)` frente a `(9, 6)`, `(13, 16)`): el primer par queda a ambos lados, pero `d3 = 0`, porque `(9, 6)` está justo en la recta que pasa por `(3, 3)` y `(15, 9)`. Está dentro de la caja de ese segmento, así que se tocan: `[1, 1, -1, 0, 1]`.

"Uno al lado del otro" (`(2, 2)`, `(8, 14)` frente a `(10, 2)`, `(16, 14)`): los dos extremos de cada segmento quedan a un mismo lado del otro, y ninguna prueba da 0. La salida es `[0, 1, 1, -1, -1]`.

## Pasos

1. Calcula `d1 = orient(p3, p4, p1)`, `d2 = orient(p3, p4, p2)`, `d3 = orient(p1, p2, p3)`, `d4 = orient(p1, p2, p4)`.
2. Si `d1 · d2 < 0` y `d3 · d4 < 0`, se cruzan: devuelve 1.
3. Para cada prueba que dé 0, si su punto está dentro de la caja envolvente del otro segmento, se tocan: devuelve 1.
4. Si no, devuelve 0.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(1)`: cuatro productos cruzados y unas pocas comparaciones |
| Espacio extra | `O(1)` |

## Errores comunes

- **Saltarse la comprobación de la caja.** Dos segmentos en una misma recta, como `(1, 1)–(3, 3)` y `(5, 5)–(8, 8)`, dan cuatro ceros pero nunca se tocan; solo la comprobación de la caja dice `[0, 0, 0, 0, 0]`.
- **Usar `≤ 0` en la prueba de cruce.** Un cero ahí es un posible contacto, no un cruce; sigue necesitando la comprobación de la caja.
- **Pendientes en coma flotante.** Comparar pendientes con divisiones invita a errores de redondeo y a dividir entre cero con segmentos verticales; los productos cruzados enteros son exactos.

## Cuándo usarlo

Úsalo para comprobar si caminos, paredes o aristas se encuentran: pruebas de colisión, si un polígono se corta a sí mismo o si un camino atraviesa una cerca. El problema enlazado, Caminos que se cruzan, pide justo esto para muchos pares, incluidos solapamientos y extremos compartidos.

## Temas relacionados

- [Envolvente convexa (cadena monótona)](learn:math-geometry/convex-hull): el mismo producto cruzado decide cada giro de la envolvente.
- [Fusionar intervalos](learn:greedy/merge-intervals): el primo unidimensional, que decide cuándo se solapan dos rangos sobre una recta.
