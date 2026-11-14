## La idea

Una lista de `n` valores tiene `2ⁿ` subconjuntos, porque cada valor está dentro o fuera. La **vuelta atrás** los enumera construyendo una respuesta cada vez en una única lista compartida, `chosen`, con tres movimientos que se repiten en cada nivel: **elegir** (añadir un valor), **explorar** (llamarse a sí misma para ampliar esa elección) y **deshacer** (quitar otra vez el valor). Como cada llamada deja `chosen` exactamente como lo encontró, una sola lista sirve para toda la búsqueda; solo se copian las respuestas que se apuntan.

Es [recursión](learn:foundations/recursion-call-stack) normal con un matiz: las llamadas comparten su estado y lo dejan limpio al terminar. La aplicación dibuja la búsqueda como un esquema sangrado, una línea por llamada; la sangría es el número de valores que hay en `chosen`.

Aquí cada llamada, `grow(start)`, primero **apunta** una copia de `chosen`, porque cada elección parcial ya es un subconjunto. Después prueba cada posición `k` desde `start`: elige `values[k]`, llama a `grow(k + 1)` y deshace. Pasar `k + 1` hace que los valores se añadan solo en el orden de la entrada, así que cada subconjunto se construye una sola vez y en un solo orden. La salida es el número de subconjuntos y después cada uno como su tamaño seguido de sus valores. Los valores se eligen por posición, así que valores iguales en posiciones distintas dan subconjuntos repetidos.

Otra forma de enumerar subconjuntos es una **máscara de bits**: cuenta de `0` a `2ⁿ − 1` y deja que el bit `k` del contador diga si el valor `k` está dentro. No necesita recursión, pero recorre los subconjuntos en otro orden y no puede saltarse una rama entera de golpe.

### La entrada por defecto

`[1, 2, 3]`. `grow(0)` apunta `[]` y elige 1; `grow(1)` apunta `[1]` y elige 2; `grow(2)` apunta `[1, 2]` y elige 3; `grow(3)` apunta `[1, 2, 3]` y no le queda nada por probar. De vuelta en `grow(1)`, se deshace el 2 y se elige el 3: `[1, 3]`. Después se deshace el 1 y `grow(0)` pasa a `[2]`, `[2, 3]` y por último `[3]`.

Ocho llamadas, ocho subconjuntos. La salida es `[8, 0, 1, 1, 2, 1, 2, 3, 1, 2, 3, 2, 1, 3, 1, 2, 2, 2, 3, 1, 3]`.

## Pasos

1. Empieza con `chosen` vacío y llama a `grow(0)`.
2. En `grow(start)`, apunta una copia de `chosen`.
3. Para cada posición `k` de `start` a `n − 1`: añade `values[k]`, llama a `grow(k + 1)` y después quítalo.
4. Devuelve el número de subconjuntos y después cada uno como su tamaño y sus valores.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n · 2ⁿ)`: `2ⁿ` subconjuntos, cada uno copiado en hasta `n` pasos |
| Espacio extra | `O(n)` para la profundidad de llamadas y `chosen`, aparte de la salida |

## Errores comunes

- **Apuntar el propio `chosen`.** Entonces todas las entradas son la misma lista, que está vacía al acabar la búsqueda. Apunta una copia.
- **Olvidar deshacer.** Un valor que se queda en `chosen` se cuela en todos los subconjuntos posteriores.
- **Recorrer desde 0 en vez de desde `start`.** La posición inicial es lo que mantiene cada subconjunto en un solo orden; sin ella, los valores se repiten y la recursión no termina nunca.
- **El crecimiento.** Cada valor más duplica el trabajo: 20 valores ya dan más de un millón de subconjuntos.

## Cuándo usarlo

Úsalo cuando haya que enumerar o comprobar todos los conjuntos de decisiones de sí o no, sobre todo cuando una elección parcial se puede descartar pronto, lo que se salta toda su rama. Los problemas enlazados, Elige el conjunto y Menús con máscaras, se construyen con decisiones así.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): las llamadas que la vuelta atrás apila y deshace.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): subconjuntos como los bits de un número.
- [Viajante de comercio (PD con máscaras)](learn:dynamic-programming/bitmask-tsp): una tabla indexada por subconjuntos.
- [Permutaciones](learn:backtracking/permutations): el mismo elegir y deshacer, para cada orden.
- [Suma de combinaciones](learn:backtracking/combination-sum): subconjuntos que deben llegar a un total, con poda.
