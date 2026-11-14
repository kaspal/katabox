## La idea

Para cada posición, busca el siguiente valor a su derecha que sea mayor, o `-1` si no hay ninguno. Recorrer hacia la derecha desde cada posición cuesta `O(n²)` en una lista descendente. Una pila monótona responde a todas las posiciones en una sola pasada.

Recorre de izquierda a derecha. La pila guarda las posiciones que todavía **esperan** un valor mayor. Sus valores bajan desde el fondo de la pila hasta la cima: si después de una posición en espera hubiera llegado un valor mayor, ya la habría respondido. Cuando llega un valor nuevo `a[i]`, es la respuesta de todas las posiciones en espera de la cima cuyo valor es menor. Sácalas una a una, escribiendo `a[i]` como su respuesta, y para en la primera cima que no sea menor: todo lo que hay debajo es aún mayor. Después mete `i`, que ahora también espera. Las posiciones que quedan en la pila al final nunca encontraron un valor mayor y se quedan con `-1`.

## Pasos

1. Llena `result` con `-1` y empieza con una `stack` vacía.
2. Para cada `i`, de izquierda a derecha:
3. Mientras la pila no esté vacía y `a[stack[-1]] < a[i]`, saca la posición de la cima y pon su `result` a `a[i]`.
4. Mete `i` en la pila.
5. Tras el último valor, devuelve `result`; las posiciones que siguen en la pila se quedan con `-1`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n)` (lista ascendente: cada valor saca al anterior) |
| Caso medio | `O(n)` |
| Peor caso | `O(n)`: cada posición entra una vez y sale como mucho una vez |
| Espacio extra | `O(n)`: la pila (una lista descendente deja en ella todas las posiciones) |

Igual que en la ventana deslizante, el bucle interior no lo convierte en `O(n²)`: en toda la pasada solo hay `n` entradas, así que puede haber como mucho `n` salidas.

## Errores comunes

- **Guardar valores en lugar de posiciones.** El ejemplo escribe la respuesta en `result[stack.pop()]`, que necesita la posición. Guarda posiciones en la pila y lee sus valores a través de `a`.
- **`<` frente a `<=`.** Con `<`, un valor igual no cuenta como mayor, así que los valores iguales esperan juntos. Usa `<=` solo si la tarea pide el siguiente valor mayor **o igual**.
- **Olvidar los que sobran.** Los valores que se quedan en la pila no tienen respuesta. Llenar `result` con `-1` de antemano los cubre sin un segundo bucle.

## Cuándo usarlo

Úsala siempre que cada elemento necesite su vecino mayor (o menor) más cercano: cuántos días faltan para uno más cálido, qué edificio tapa primero la vista hacia la derecha, el tramo de días de un precio en bolsa o el rectángulo más grande bajo un diagrama de barras. Para el "mayor anterior", recorre igual pero lee la cima de la pila antes de meter; para el "siguiente menor", invierte la comparación.

## Temas relacionados

- [Ventana deslizante](learn:searching/sliding-window): otra pasada en la que cada índice entra y sale una vez.
- [Coste amortizado: el arreglo dinámico](learn:foundations/dynamic-array): el mismo argumento de "barato de media en toda la pasada".
- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): otra pila, un marco por llamada.
- Cola monótona (próximamente): la misma idea, sacando por los dos extremos.
