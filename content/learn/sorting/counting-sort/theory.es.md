## La idea

Cuando los valores son números enteros pequeños, no hace falta compararlos. La ordenación por conteo tiene una **cubeta** (un contador) por cada valor posible, aquí de 0 a 9. Una primera pasada echa cada valor en su cubeta, así que la altura de una cubeta es cuántas veces aparece su valor. Una segunda pasada vacía las cubetas en orden de valor y escribe cada valor tantas veces como se contó.

Este ejemplo escribe los propios valores, que es todo lo que necesitan los números sueltos. Cuando cada valor lleva un registro asociado (un nombre con una puntuación), la versión habitual es estable: convierte los recuentos en sumas prefijas, para que cada valor sepa dónde va su primera copia, y luego recorre la entrada de izquierda a derecha y pone cada registro en el siguiente hueco libre de su valor, en un arreglo de salida aparte. Los valores iguales conservan el orden de la entrada.

## Pasos

1. Crea `counts` con una celda por cada valor de 0 a `max(a)`, todas a cero.
2. Para cada valor `v` del arreglo, suma uno a `counts[v]`.
3. Pon `k = 0`, la siguiente posición donde escribir.
4. Para cada valor `v` desde 0 hacia arriba, escribe `v` en `a[k]` `counts[v]` veces, avanzando `k` cada vez.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n + k)` (`k` = número de valores posibles) |
| Caso medio | `O(n + k)` |
| Peor caso | `O(n + k)` |
| Espacio extra | `O(k)` para los recuentos |
| Estable | Sí, en la versión con sumas prefijas |
| En el sitio | No |

## Errores comunes

- **Un rango enorme.** El coste crece con `k`, no solo con `n`. Diez valores entre 0 y mil millones necesitarían mil millones de contadores. La ordenación por conteo solo compensa cuando `k` no es mucho mayor que `n`.
- **Valores negativos.** `counts[v]` necesita `v ≥ 0`. Desplaza antes todos los valores restando el mínimo: `counts[v - min]`.
- **Perder los registros.** Reescribir los valores a partir de los recuentos tira todo lo que iba asociado a ellos. Para mover registros completos, usa la versión con sumas prefijas.

## Cuándo usarlo

Úsala con claves enteras pequeñas: notas de 0 a 10, edades, letras de un alfabeto, cifras. Nunca compara dos valores, así que no está atada al límite `O(n log n)` de las ordenaciones por comparación. Además es el paso interno de la ordenación por residuos, que ordena números más grandes cifra a cifra con una pasada de conteo estable.

## Temas relacionados

- [Sumas prefijas](learn:foundations/prefix-sums): cómo los recuentos se convierten en posiciones en la versión estable.
- [Ordenación por residuos](learn:sorting/radix-sort): contar, cifra a cifra.
- [Ordenación por cubetas](learn:sorting/bucket-sort): cubetas que guardan un tramo de valores en lugar de uno solo.
