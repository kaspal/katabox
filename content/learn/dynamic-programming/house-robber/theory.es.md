## La idea

En una fila de casas cada una guarda un botín, pero nunca se pueden robar dos casas **vecinas**. ¿Cuál es el total más grande y qué casas lo dan? Probar todos los conjuntos permitidos crece de forma exponencial; tomar una casa sí y otra no es sencillo, pero a menudo pierde.

Sea `best[i]` el mayor total con las **primeras `i` casas**. Para la casa `i - 1`, la última de ellas, solo hay dos opciones:

- **saltarla**: el total es `best[i - 1]`;
- **tomarla**: hay que saltarse su vecina, así que el total es `best[i - 2] + valor`.

`best[i]` es el mayor de los dos. Los casos base son `best[0] = 0` (sin casas) y `best[1]` = el valor de la primera casa. La tabla guarda solo totales, así que una **reconstrucción** recupera las casas: se recorre hacia atrás desde `best[n]`. Si `best[i]` es igual a `best[i - 1]`, la casa `i - 1` se puede saltar, así que se retrocede uno; si no, se tomó, así que se apunta y se retrocede dos, por encima de su vecina. En caso de empate el ejemplo prefiere saltar. Devuelve `best[n]` y después las casas elegidas (empezando en 0) de izquierda a derecha.

### La entrada por defecto

Casas `6, 3, 10, 8, 2, 9, 4`. `best[1] = 6`. Casa 1 (3): tomarla da 0 + 3 y saltarla mantiene 6, así que `best[2] = 6`. Casa 2 (10): 6 + 10 = 16 gana a 6. Casa 3 (8): 6 + 8 = 14 pierde frente a 16. Casa 4 (2): 16 + 2 = 18. Casa 5 (9): 16 + 9 = 25 gana a 18. Casa 6 (4): 18 + 4 = 22 pierde frente a 25. La tabla es `[0, 6, 6, 16, 16, 18, 25, 25]`.

Reconstrucción desde 7: `best[7] = best[6]`, así que la casa 6 se salta. `best[6] ≠ best[5]`: se toma la casa 5 y se salta a 4. `best[4] = best[3]`: se salta la casa 3. `best[3] ≠ best[2]`: se toma la casa 2 y se salta a 1. `best[1] ≠ best[0]`: se toma la casa 0.

La salida es `[25, 0, 2, 5]`: 6 + 10 + 9. Una casa sí y otra no daría solo 22 (casas 0, 2, 4, 6) o 20 (casas 1, 3, 5).

## Pasos

1. Pon `best[0] = 0` y `best[1]` al primer valor.
2. Para `i` de 2 a `n`, pon `best[i] = max(best[i - 1], best[i - 2] + values[i - 1])`.
3. Reconstrucción: empieza en `i = n`. Mientras `i > 0`: si `best[i] = best[i - 1]`, retrocede uno; si no, apunta la casa `i - 1` y retrocede dos.
4. Devuelve `best[n]` y las casas apuntadas, en orden inverso.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n)`: una decisión por casa, más una reconstrucción de como mucho `n` pasos |
| Espacio extra | `O(n)` para la tabla, que la reconstrucción necesita |

## Errores comunes

- **Retroceder uno tras tomar una casa.** La vecina de una casa tomada se saltó por definición, así que la reconstrucción debe retroceder dos.
- **Confundir los índices.** `best` tiene `n + 1` celdas, y `best[i]` habla de la casa `i - 1`.
- **Esperar una única respuesta.** Los empates pueden hacer que dos conjuntos distintos sean igual de buenos; preferir saltar elige uno de ellos.
- **Tirar la tabla demasiado pronto.** Con dos variables basta para el mejor total, pero las casas necesitan la tabla entera.

## Cuándo usarlo

La elección de saltar o tomar con la regla "nunca dos seguidas" sirve para planificar turnos, elegir asientos no contiguos o cualquier fila de decisiones en la que usar una bloquea a su vecina. Es además el ejemplo más pequeño de reconstrucción.

## Temas relacionados

- [Subir escaleras](learn:dynamic-programming/climbing-stairs): la misma mirada dos celdas atrás, sumando en vez de elegir.
- [Mochila 0/1](learn:dynamic-programming/knapsack-01): tomar o dejar cada objeto, con una capacidad en vez de vecinas.
- [Subarreglo de suma máxima (Kadane)](learn:dynamic-programming/maximum-subarray): otra regla sobre una fila de valores.
- Subsecuencia creciente más larga (próximamente): elegir elementos en orden con una reconstrucción.
