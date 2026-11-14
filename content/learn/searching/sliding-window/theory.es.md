## La idea

Busca el tramo más largo de valores vecinos cuya suma sea como mucho `limit`, en una lista de valores **positivos**. Comprobar todos los tramos cuesta `O(n²)` o más. Una ventana deslizante guarda un solo tramo, `a[lo..hi]`, y su suma en `total`, y sus bordes solo avanzan.

En cada paso, el borde derecho `hi` incorpora un valor más. Si con eso `total` pasa del límite, el borde izquierdo `lo` va soltando valores hasta que la ventana vuelve a caber. Entonces la ventana es candidata: si es más larga que la mejor hasta ahora, guarda su longitud y su inicio. El ejemplo devuelve `[best, best_start]`.

¿Por qué el borde izquierdo nunca necesita retroceder? Porque todos los valores son positivos: si `a[lo..hi]` pasa del límite, cualquier tramo que empiece en `lo` y termine más a la derecha también. Así que `lo` ya no vuelve a ser un inicio útil una vez soltado.

## Pasos

1. Pon `best = 0`, `best_start = 0`, `total = 0` y `lo = 0`.
2. Para cada `hi`, de izquierda a derecha, suma `a[hi]` a `total`.
3. Mientras `total > limit`, resta `a[lo]` y mueve `lo` un lugar a la derecha.
4. Si `hi - lo + 1 > best`, guarda `best = hi - lo + 1` y `best_start = lo`.
5. Tras el último valor, devuelve `[best, best_start]`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n)` |
| Caso medio | `O(n)` |
| Peor caso | `O(n)`: `hi` se mueve `n` veces y `lo` como mucho `n`, `2n` movimientos en total |
| Espacio extra | `O(1)`: dos bordes y una suma acumulada |

El bucle `while` interior parece que podría convertirlo en `O(n²)`, pero no puede: `lo` solo avanza, así que en toda la pasada se mueve como mucho `n` veces.

## Errores comunes

- **Valores negativos (o ceros, en algunas tareas).** Con un valor negativo, añadir más por la derecha puede bajar una suma que se pasaba, así que soltar `lo` antes de tiempo puede tirar el mejor tramo. El método necesita valores positivos; si puede haber negativos, usa métodos basados en sumas prefijas.
- **Recalcular la suma.** Sumar `a[lo..hi]` desde cero en cada paso vuelve a `O(n²)`. Mantén `total` al día: suma por la derecha, resta por la izquierda.

## Cuándo usarlo

Úsala para preguntas del tipo "el tramo más largo o más corto tal que…" en las que alargar el tramo solo empeora (o mejora) las cosas en un sentido: el tramo más largo dentro de un presupuesto, el tramo más corto que alcanza un total, la subcadena más larga sin letras repetidas (con un recuento de letras en la ventana en lugar de una suma). Las ventanas de tamaño fijo, como el mejor total de 7 días seguidos, son el caso más sencillo: entra uno por la derecha, sale uno por la izquierda.

## Temas relacionados

- [Dos punteros](learn:searching/two-pointers): los mismos dos índices, empezando en extremos opuestos.
- [Sumas prefijas](learn:foundations/prefix-sums): sumas de cualquier tramo, también con valores negativos.
- Cola monótona (próximamente): el máximo dentro de una ventana deslizante.
