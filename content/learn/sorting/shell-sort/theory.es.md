## La idea

La ordenación por inserción mueve un valor un solo lugar en cada desplazamiento, así que un valor pequeño cerca del final necesita muchos desplazamientos para llegar a la izquierda. La ordenación de Shell hace primero la ordenación por inserción entre valores separados por un **salto**, de modo que un valor puede llegar lejos en un solo movimiento. Este ejemplo usa los saltos originales de Shell: la mitad de la longitud, luego la mitad otra vez, hasta llegar a 1. La pasada con salto 1 es la ordenación por inserción de siempre, pero para entonces el arreglo está casi ordenado y queda poco trabajo.

## Pasos

1. Empieza con `gap = len(a) // 2`.
2. Para cada `i` desde `gap` hasta el final, saca `a[i]` como clave (`key`).
3. Mientras el valor situado `gap` posiciones a la izquierda del hueco sea mayor que la clave, muévelo `gap` posiciones a la derecha; el hueco se mueve `gap` posiciones a la izquierda.
4. Coloca la clave en el hueco.
5. Reduce el salto a la mitad y repite. La pasada con salto 1 termina la ordenación.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n log n)` (entrada ordenada: una comparación por valor en cada pasada) |
| Caso medio | Depende de la secuencia de saltos |
| Peor caso | `O(n²)` con los saltos de Shell |
| Espacio extra | `O(1)` |
| Estable | No: un salto largo puede adelantar a un valor igual |
| En el sitio | Sí |

## Errores comunes

- **Comparar con el vecino.** La clave se compara con el valor que está `gap` posiciones a la izquierda, `a[j - gap]`, no con `a[j - 1]`, y el bucle debe comprobar `j >= gap` para no leer nunca antes de la posición 0.
- **Los saltos de Shell pueden ser malos.** Cuando la longitud es una potencia de dos, todos los saltos son pares hasta la última pasada, así que los valores de las posiciones pares nunca se encuentran con los de las impares antes del salto 1. Las entradas pensadas para aprovechar eso llevan el trabajo a `O(n²)`.
- **Hay secuencias mejores.** Otras listas de saltos, como la de Knuth 1, 4, 13, 40, … o listas obtenidas por experimentación, evitan esa trampa y son mucho más rápidas en la práctica. Solo cambian los saltos; el bucle es el mismo.

## Cuándo usarlo

La ordenación de Shell es corta, trabaja en el sitio, sin recursión ni memoria extra, lo que ayuda en sistemas embebidos pequeños o con arreglos medianos cuando no tienes a mano una ordenación de biblioteca. Para entradas grandes o un límite garantizado, mejor la ordenación por mezcla o por montículo.

## Temas relacionados

- [Ordenación por inserción](learn:sorting/insertion-sort): cada pasada es una ordenación por inserción con pasos de `gap`.
- [Ordenación por mezcla](learn:sorting/merge-sort): un `O(n log n)` garantizado, a cambio de un búfer.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): por qué la secuencia de saltos cambia el crecimiento.
