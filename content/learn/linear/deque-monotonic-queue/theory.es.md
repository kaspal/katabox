## La idea

Una **bicola** (cola de dos extremos) te deja meter y sacar tanto por el principio como por el final, cada cosa en `O(1)`. Su truco más conocido es la **cola monótona**, que encuentra el máximo de cada ventana de `k` valores en una sola pasada.

La bicola guarda **posiciones**, no valores, y mantiene sus valores decrecientes desde el principio hasta el final. Cuando llega un valor nuevo:

- Cada valor del final que sea **menor o igual** que él ya nunca podrá ser el máximo de una ventana: el nuevo es al menos igual de grande y se queda más tiempo en la ventana. Sácalos por el final y luego mete la nueva posición.
- Si la posición del principio ya se ha salido de la ventana, sácala por el principio.
- Ahora el principio es el mayor valor de la ventana.

### Por qué es lineal

Una sola llegada puede sacar varios valores, así que un paso puede parecer caro. Pero cada posición entra exactamente una vez y sale como mucho una vez, así que todas las salidas juntas cuestan como mucho `n`. La pasada entera es `O(n)`, frente a `O(n · k)` si recorres cada ventana.

## Pasos

1. Empieza con una bicola `dq` vacía y una fila de salida vacía.
2. Para cada posición `i`: mientras `dq` no esté vacía y `a[dq.back] <= a[i]`, saca el final.
3. Mete `i` por el final.
4. Si la posición del principio es `<= i − k`, está fuera de la ventana: saca el principio.
5. En cuanto `i >= k − 1`, la ventana `i − k + 1..i` está completa: saca `a[dq.front]` a la salida.

## Coste

| Medida | Coste |
|---|---|
| Meter o sacar por cualquier extremo | `O(1)` |
| Pasada entera sobre `n` valores | `O(n)`: cada posición entra y sale una vez |
| Espacio extra | `O(k)`: la bicola tiene como mucho `k + 1` posiciones, por un momento |

## Errores comunes

- **Guardar valores en lugar de posiciones.** Solo con los valores no puedes saber cuándo el principio ha salido de la ventana. Guarda posiciones y lee los valores a través de ellas.
- **Sacar solo los valores estrictamente menores.** Con `<` se quedan los valores iguales. La respuesta sigue siendo correcta, pero la bicola crece para nada; `<=` la mantiene ligera.
- **Una lista como bicola.** Quitar el principio de una lista de Python o de un arreglo de JavaScript es `O(n)`. Usa `collections.deque` o un índice de principio sobre un arreglo.

## Cuándo usarlo

Usa una cola monótona para máximos o mínimos de una ventana que se desliza de paso en paso: la hora más calurosa de cada tramo de 3 horas, el precio más barato de la última semana. Para el mínimo, invierte la comparación para que los valores crezcan del principio al final.

## Temas relacionados

- [Ventana deslizante](learn:searching/sliding-window): la ventana de la que esta bicola guarda el máximo.
- [Pila monótona](learn:searching/monotonic-stack): la misma poda con un solo extremo abierto.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): una cola normal en un arreglo fijo.
- [Lista doblemente enlazada](learn:linear/doubly-linked-list): una forma de construir una bicola.
- Montículo binario (próximamente): máximos de ventana en `O(n log n)`.
