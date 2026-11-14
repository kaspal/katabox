## La idea

Coloca `n` reinas en un tablero de `n × n` de modo que ninguna ataque a otra: no puede haber dos en la misma fila, columna o diagonal. Como cada fila lleva exactamente una reina, la búsqueda va **fila a fila**, de arriba abajo, y el estado es solo `cols`, donde `cols[r]` es la columna de la reina de la fila `r`.

Para la siguiente fila `r`, una columna `c` es **segura** cuando cada reina anterior `(rr, cc)` cumple `cc ≠ c` (otra columna) y `|c − cc| ≠ r − rr` (no está en diagonal: en una diagonal, la distancia en columnas es igual a la distancia en filas). El ejemplo prueba las columnas de izquierda a derecha; para cada una segura coloca la reina, baja a la fila siguiente y después la quita. Cuando las `n` filas están llenas, se apuntan las columnas. La búsqueda no se detiene en la primera solución: **las cuenta todas**. La salida es el número de soluciones y después las columnas de la primera, fila a fila. El tamaño del tablero se escribe como un número de 4 a 6; el tablero sombrea las casillas que atacan las reinas colocadas.

### La entrada por defecto

`n = 5`. La fila 0 toma la columna 0. En la fila 1, la columna 0 comparte columna y la columna 1 una diagonal, así que la reina va a la columna 2. En la fila 2, las columnas 0 y 2 están ocupadas y las columnas 1 y 3 tocan en diagonal a la reina de la fila 1; la columna 4 es segura. La fila 3 toma la columna 1, y la fila 4 encuentra usadas las columnas 0 a 2, así que toma la columna 3. La primera solución, `[0, 2, 4, 1, 3]`, aparece sin un solo callejón sin salida. La búsqueda sigue por todas las demás ramas y encuentra 10 soluciones en total.

La salida es `[10, 0, 2, 4, 1, 3]`. Más grande no siempre es más: `n = 4` tiene 2 soluciones y `n = 6` solo 4.

## Pasos

1. Empieza con `cols` vacío y llama a `place()`.
2. Si `cols` tiene `n` columnas, apunta una copia y vuelve.
3. Si no, para cada columna `c` de 0 a `n − 1`: si es segura frente a todas las reinas de `cols`, añádela, llama a `place()` y quítala.
4. Devuelve el número de soluciones y después las columnas de la primera.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | Cota `O(n!)`: solo la regla de las columnas deja `n` opciones en la fila 0, `n − 1` en la fila 1, y así sucesivamente |
| Caso medio | Mucho menos que `n!` gracias a la poda: las diagonales cortan casi todas las ramas tras unas pocas filas |
| Peor caso | `O(n!)`, y cada comprobación revisa hasta `n` reinas |
| Espacio extra | `O(n)` para `cols` y la profundidad de llamadas |

## Errores comunes

- **La prueba de la diagonal.** La distancia en columnas necesita el valor absoluto; la distancia en filas ya es positiva porque las filas anteriores están arriba.
- **Detenerse en la primera solución.** Vale para encontrar una, pero el recuento saldría mal.
- **Imágenes en espejo.** Las 10 soluciones para `n = 5` incluyen copias reflejadas y giradas unas de otras; todas se cuentan.

## Cuándo usarlo

Las N reinas son el modelo de la **búsqueda con restricciones**: coloca una pieza cada vez y descarta una colocación parcial en cuanto rompe una regla. Conviene conocer dos mejoras. Guardar conjuntos (o máscaras de bits) de columnas usadas y de las dos direcciones diagonales (`r + c` y `r − c` son constantes a lo largo de cada una) hace que la comprobación cueste tiempo constante. Y la simetría reduce el trabajo a la mitad: cada solución tiene su imagen en espejo, así que basta con probar la mitad izquierda de la fila 0 y duplicar el recuento; cuando `n` es impar, la columna central se busca aparte.

## Temas relacionados

- [Permutaciones](learn:backtracking/permutations): una reina por fila y por columna es una permutación de las columnas.
- [Sudoku](learn:backtracking/sudoku): el mismo colocar, comprobar y quitar en una cuadrícula de números.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): la herramienta de los resolvedores de reinas más rápidos.
- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): la vuelta atrás es un recorrido en profundidad por las elecciones.
