## Enfoque: dos punteros desde ambos extremos

Pon `lo` en el primer artículo y `hi` en el último, y guarda el mejor total visto (empezando en `-1`).

- Si `prices[lo] + prices[hi] > budget`, el artículo de `hi` es demasiado caro incluso con la pareja más barata que queda, así que se descarta: `hi -= 1`.
- Si no, el par cabe. Anota el total; cualquier otra pareja de `lo` a la izquierda de `hi` da un total menor, así que `lo` ya encontró su mejor pareja: `lo += 1`.

Cada paso descarta un artículo, de modo que el bucle da menos de n vueltas.

### Paso a paso

`prices = [3, 5, 8, 12, 20]`, `budget = 19`:

| lo | hi | total | acción | mejor |
| --- | --- | --- | --- | --- |
| 3 | 20 | 23 | se pasa: hi a la izquierda | -1 |
| 3 | 12 | 15 | cabe: lo a la derecha | 15 |
| 5 | 12 | 17 | cabe: lo a la derecha | 17 |
| 8 | 12 | 20 | se pasa: hi a la izquierda | 17 |

`lo` y `hi` se encuentran; la respuesta es 17.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Revisar todos los pares cuesta O(n²), demasiado lento para 5·10⁴ precios.
