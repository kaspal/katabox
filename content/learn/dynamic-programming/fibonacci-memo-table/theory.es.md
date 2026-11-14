## La idea

La **programación dinámica** resuelve un problema respondiendo una sola vez a versiones más pequeñas de él, guardando cada respuesta y construyendo las respuestas grandes a partir de las guardadas. Merece la pena cuando se cumplen dos cosas. **Subproblemas superpuestos**: la misma pregunta más pequeña aparece una y otra vez. **Subestructura óptima**: la respuesta de un caso grande se arma con las respuestas de casos más pequeños.

Los números de Fibonacci muestran ambas. `fib(0) = 0`, `fib(1) = 1`, y cada número siguiente es la suma de los dos anteriores. La recursión directa sigue esa regla al pie de la letra: `fib(8)` llama a `fib(7)` y a `fib(6)`, pero `fib(7)` vuelve a llamar a `fib(6)`, y las repeticiones se multiplican hasta abajo. El número de llamadas crece como los propios números de Fibonacci, unas 1,6 veces más por cada paso de `n`: 67 llamadas para `fib(8)`, más de dos millones para `fib(30)`.

Hay dos formas de evitar ese derroche. **De arriba abajo (memoización)** mantiene la recursión y añade una memoria, un diccionario de `n` a su respuesta. Cada llamada mira primero en la memoria; si la respuesta está, es un **acierto** y la recursión se corta en el acto. Si no, calcula la respuesta y la **guarda**. **De abajo arriba (tabulación)** prescinde de la recursión: llena una tabla desde el caso más pequeño hacia arriba, `table[i] = table[i - 1] + table[i - 2]`, así que cada celda que lee ya está llena.

El ejemplo hace las dos cosas. Devuelve `fib(n)` calculado con la memoria y después la tabla completa `table[0..n]`.

### La entrada por defecto

`n = 8`. La versión con memoria baja por la primera rama: `fib(8)` llama a `fib(7)`, que llama a `fib(6)`, y así hasta que `fib(2)` llama a `fib(1)` y a `fib(0)`, que guardan 1 y 0. A la vuelta, cada `fib(k)` de 3 a 8 hace su segunda llamada, `fib(k - 2)`, y la encuentra en la memoria. Son 9 guardados y 6 aciertos: 15 llamadas en vez de 67. La versión con tabla llena las celdas 2 a 8 de izquierda a derecha, 7 sumas en total.

La salida es `[21, 0, 1, 1, 2, 3, 5, 8, 13, 21]`: primero `fib(8) = 21` y después la tabla.

## Pasos

1. De arriba abajo: si `n` está en la memoria, devuelve la respuesta guardada (un acierto).
2. Si no, la respuesta es `n` cuando `n < 2` y, si no, `fib(n - 1) + fib(n - 2)`. Guárdala en la memoria y devuélvela.
3. De abajo arriba: crea una tabla de `n + 1` ceros y pon `table[1] = 1` cuando `n > 0`.
4. Para `i` de 2 a `n`, pon `table[i] = table[i - 1] + table[i - 2]`.

## Coste

| Caso | Coste |
|---|---|
| Recursión directa | `O(φⁿ)` llamadas, con `φ ≈ 1,618` |
| Memoria o tabla | `O(n)`: cada valor se calcula una vez |
| Espacio extra | `O(n)`: la memoria y la pila de llamadas, o la tabla |

## Errores comunes

- **Consultar una memoria en la que nunca se escribe.** Sin el guardado, vuelve a ser recursión directa.
- **Recursión profunda.** De arriba abajo se llega a `n` llamadas de profundidad, y Python se detiene hacia las 1000 por defecto; la tabla no tiene ese límite.
- **Llenar en el orden equivocado.** De abajo arriba solo funciona si cada celda que lee la fórmula se llenó antes. Aquí eso significa de izquierda a derecha.

## Cuándo usarlo

Recurre a la programación dinámica cuando una solución recursiva hace las mismas preguntas una y otra vez. De arriba abajo es cómodo cuando la recursión ya está escrita o solo hacen falta algunos subproblemas; de abajo arriba evita la pila de llamadas y deja claro el orden de llenado. Cuando cada celda lee solo las últimas, quédate solo con ellas: para Fibonacci bastan dos variables.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): las llamadas que la memoria corta.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): crecimiento exponencial frente a lineal.
- [Subir escaleras](learn:dynamic-programming/climbing-stairs): la misma regla disfrazada.
- Subsecuencia común más larga (próximamente): una tabla de dos dimensiones.
