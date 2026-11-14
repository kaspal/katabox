## La idea

Hay `n` discos en la varilla 1, el más grande abajo. Hay que llevarlos todos a la varilla 3, de uno en uno y sin poner nunca un disco grande sobre uno pequeño. El truco es pensar solo en el disco más grande: solo puede moverse cuando todos los más pequeños están apartados, en la varilla libre. Así que `hanoi(n, source, target, spare, moves)` lleva los `n - 1` discos pequeños a la varilla libre, mueve el disco grande y vuelve a poner los `n - 1` discos encima. Mover `n - 1` discos es el mismo problema, un tamaño más pequeño.

## Pasos

1. Si `n == 0`, no hay nada que mover: vuelve.
2. `hanoi(n - 1, source, spare, target, moves)`: aparta los discos pequeños a la varilla libre.
3. Mueve el disco `n` de `source` a `target` y anota el movimiento.
4. `hanoi(n - 1, spare, target, source, moves)`: vuelve a apilar los discos pequeños encima.
5. `solve(n)` lo pone todo en marcha con `hanoi(n, 1, 3, 2, moves)`.

## Coste

| Medida | Coste |
|---|---|
| Movimientos | `2ⁿ − 1`, el mínimo posible |
| Llamadas | `2ⁿ⁺¹ − 1`, contando las llamadas con `n == 0` que vuelven enseguida |
| Tiempo | `O(2ⁿ)` |
| Profundidad de la pila | `O(n)`: como mucho `n + 1` marcos |

## Errores comunes

- **Cambiar los papeles de las varillas.** La primera llamada recursiva va a `spare` (con `target` como varilla libre); la segunda va de `spare` a `target`. Si los mezclas, las llamadas parecen válidas pero los discos acaban en el sitio equivocado.
- **Esperar un atajo.** El disco más grande tiene que moverse al menos una vez, y cada vez los `n - 1` discos de encima deben irse y volver, así que `2ⁿ − 1` movimientos es el mínimo. Cada disco más duplica el trabajo: 20 discos necesitan más de un millón de movimientos.
- **El primer movimiento depende de `n`.** Con un número impar de discos, el más pequeño va primero a la varilla 3; con un número par, a la 2. Prueba con `n = 2` y `n = 3`.

## Cuándo usarlo

Las Torres de Hanói son el ejercicio clásico para aprender a confiar en la recursión: da por hecho que la llamada más pequeña funciona y planifica solo el paso grande. La misma forma, "resuelve `n - 1`, da un paso, resuelve `n - 1` otra vez", aparece en la vuelta atrás recursiva y en argumentos de conteo. También es un ejemplo claro de crecimiento exponencial que ningún truco de programación puede evitar.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): marcos, casos base y el árbol de recursión.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): qué significa `O(2ⁿ)` en la práctica.
- Vuelta atrás (próximamente).
