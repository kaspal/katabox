## Enfoque: programación dinámica sobre las ofertas ordenadas por fin, con búsqueda binaria

Ordena las ofertas por `end` (la entrada no viene en ningún orden concreto) y guarda los fines ordenados en una lista `ends`. Sea `best[i]` el mejor plan que usa solo las `i` primeras ofertas, escrito como el par `(total, −count)`: un par mayor es un plan mejor, porque gana el total más alto y, a igual total, menos bolos dan un `−count` mayor.

Para la oferta `i` (desde 0) con `start` y `pay` hay dos opciones:

- **saltarla:** el mejor plan es `best[i]`;
- **tomarla:** todos los demás bolos del plan deben terminar, como tarde, en `start`. Entre las `i` primeras ofertas son exactamente las `j` primeras, con `j = bisect_right(ends, start, 0, i)`, así que el mejor plan es `best[j]` más este bolo: `(total + pay, −count − 1)`.

```
best[0] = (0, 0)
best[i + 1] = max(best[i], best[j] + (pay, −1))
```

La respuesta es `best[n]`, escrita como `total` y `count`.

**Ejemplo resuelto (Ejemplo 1).** Ordenadas por fin: `(1,4,50)`, `(1,6,80)`, `(4,6,30)`, `(6,9,20)`, así que `ends = [4, 6, 6, 9]`.

| i | oferta | j | tomarla | best[i + 1] |
| --- | --- | --- | --- | --- |
| 0 | 1-4, 50 | 0 | (50, −1) | (50, −1) |
| 1 | 1-6, 80 | 0 | (80, −1) | (80, −1) |
| 2 | 4-6, 30 | 1 | (80, −2) | (80, −1) |
| 3 | 6-9, 20 | 3 | (100, −2) | (100, −2) |

En la fila 2 el plan `50 + 30` empata en total con `80`, pero tiene un bolo más, así que pierde; por eso la respuesta es `100 2` y no `100 3`.

**Por qué es correcto.** Toma el mejor plan entre las `i + 1` primeras ofertas. Si no usa la oferta `i`, es un plan entre las `i` primeras, así que vale como mucho `best[i]`. Si la usa, sus demás bolos terminan como tarde en `start`; las ofertas de entre las `i` primeras que terminan después no pueden estar en él, y al ordenar por fin todas las que terminan como tarde en `start` quedan antes de la posición `j`. Así que el resto es un plan entre las `j` primeras, que vale como mucho `best[j]`. Las dos opciones son planes válidos, así que el máximo es exacto.

El desempate puede ir dentro de la programación dinámica porque añadir un bolo cambia todos los planes de la misma forma: si `(a, −c) ≥ (a′, −c′)`, entonces `(a + pay, −c − 1) ≥ (a′ + pay, −c′ − 1)`. Un plan que es el mejor según el par lo sigue siendo después del paso, así que guardar solo el mejor par de cada prefijo no pierde nada.

**Complejidad:** O(n log n) por la ordenación y una búsqueda binaria por oferta; memoria O(n).

**El enfoque evidente es demasiado lento.** Calcular el mejor plan que termina en cada oferta probando todas las anteriores es O(n²): con 5·10⁴ ofertas son unas 1.25·10⁹ comparaciones, muy por encima del límite.

**Notas prácticas:** lee toda la entrada de una vez y escribe la única línea de una vez. Los totales llegan a 10⁵ · 10⁹ = 10¹⁴: usa enteros de 64 bits en Rust (`i64`/`u64`, no `i32`); un `Number` de TypeScript es exacto hasta 2⁵³ y los enteros de PHP son de 64 bits. Python compara tuplas directamente; en los demás lenguajes compara primero el total y después el número de bolos. La búsqueda binaria debe encontrar la primera oferta que termina **después** de `start` (`<=` en la comparación), para que un bolo que termina justo en `start` cuente como compatible.
