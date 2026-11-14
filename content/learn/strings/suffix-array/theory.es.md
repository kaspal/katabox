## La idea

Un **sufijo** es el final de un texto a partir de alguna posición. El **arreglo de sufijos** enumera las posiciones de inicio de todos los sufijos en orden alfabético. Ordenar los sufijos directamente compara trozos largos de texto una y otra vez. La **duplicación de prefijos** los ordena por su primera letra, después por sus 2 primeras letras, después 4, después 8, reutilizando el resultado de cada ronda.

Cada sufijo recibe un **rango**: rangos iguales significan "iguales hasta ahora". Al principio el rango es simplemente la primera letra. En una ronda con paso `k`, cada sufijo `i` se ordena por un par: su propio rango, que cubre sus primeras `k` letras, y el rango del sufijo `i + k`, que cubre las `k` siguientes. Juntos describen las primeras `2k` letras. Un sufijo que no tiene nada en `i + k` usa `-1`, así que un sufijo más corto va antes que uno más largo que empieza igual. Tras ordenar, los rangos se renumeran `0, 1, 2…`, dando rangos iguales a pares iguales. Cuando el último sufijo del orden tiene rango `n - 1`, todos los rangos son distintos: el orden es definitivo. Si no, `k` se duplica. La salida son las posiciones de inicio en orden. El texto se escribe como una palabra, con letras de la a a la z.

### La entrada por defecto

Texto `tatatoma`, 8 sufijos. Con `k = 1` los pares son las dos primeras letras: `ta`, `at`, `ta`, `at`, `to`, `om`, `ma` y `a` sola. Ordenados y renumerados, los rangos son `a` 0, `at` 1 (dos veces), `ma` 2, `om` 3, `ta` 4 (dos veces), `to` 5. Quedan dos empates, así que `k` pasa a 2. Ahora el sufijo 1 (`atatoma`) recibe el par (1, 1) y el sufijo 3 (`atoma`) el (1, 3), y los sufijos 0 y 2 reciben (4, 4) y (4, 5). Todos los rangos son distintos y el bucle se detiene. El orden es `a`, `atatoma`, `atoma`, `ma`, `oma`, `tatatoma`, `tatoma`, `toma`, así que la salida es `[7, 1, 3, 6, 5, 0, 2, 4]`.

## Pasos

1. Da a cada posición el rango de su letra; el orden empieza como `0, 1, …, n - 1`; `k = 1`.
2. Ordena por el par (`rank[i]`, `rank[i + k]` o `-1`).
3. Renumera: recorriendo el orden, el rango sube en uno cada vez que cambia el par.
4. Si el último rango es `n - 1`, devuelve el orden; si no, duplica `k` y vuelve al paso 2.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n log² n)`: como mucho `log n` rondas, cada una con una ordenación por comparación |
| Espacio extra | `O(n)` para los rangos y el orden |

Tras `log n` rondas cada par cubre el sufijo entero, así que todos los rangos tienen que ser distintos. Ordenar los pares con una pasada de [ordenación por conteo](learn:sorting/counting-sort) por cada clave en lugar de una ordenación por comparación baja el total a `O(n log n)`.

## Errores comunes

- **Falta la segunda mitad.** Un sufijo más corto que `2k` no tiene rango en `i + k`; necesita un valor menor que cualquier rango real, aquí `-1`.
- **Volver a comparar letras.** Cada ronda compara dos números por sufijo, nunca el texto; en eso está todo el ahorro.
- **Parar demasiado pronto.** El bucle termina solo cuando todos los rangos son distintos, no tras un número fijo de rondas.

## Cuándo usarlo

Con los sufijos en orden, todas las apariciones de un patrón quedan en un mismo bloque, que encuentra una [búsqueda binaria](learn:searching/binary-search). Un sufijo comparte su prefijo más largo con uno de sus vecinos en el arreglo, así que la subcadena repetida más larga es el prefijo más largo que comparten dos vecinos. El problema El trozo repetido, enlazado desde [Rabin-Karp](learn:strings/rabin-karp), pide justo eso.

## Temas relacionados

- [Ordenación por mezcla (Merge Sort)](learn:sorting/merge-sort): una ordenación por comparación como la que usa cada ronda.
- [Rabin-Karp](learn:strings/rabin-karp): otra forma de encontrar trozos repetidos.
- Arreglo LCP (próximamente): el prefijo común de cada par de vecinos.
- Árbol de sufijos (próximamente): un árbol con todos los sufijos, con los mismos usos.
