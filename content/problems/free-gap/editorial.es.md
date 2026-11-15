## Enfoque: ordenar los tramos y barrer con el final de lo ocupado

**Minutos, no horas de reloj.** Convierte cada `hh:mm` en `hh · 60 + mm`. Así, un tramo es un intervalo semiabierto `[start, end)` de números enteros, y la longitud de un hueco es una resta.

**El barrido.** Ordena los tramos del día por inicio. Lleva `free_from`, el primer minuto después de todo lo ocupado visto hasta ahora, que empieza en `open`, y el mejor hueco `(best_len, best_start)`, que empieza con longitud 0:

```
para (s, e) en los tramos ordenados por inicio:
    si s − free_from > best_len: best_len, best_start = s − free_from, free_from
    free_from = max(free_from, e)
si close − free_from > best_len: best_len, best_start = close − free_from, free_from
```

El `max` resuelve los solapamientos: un tramo que termina dentro de lo ya ocupado no hace retroceder `free_from`.

Ejemplo 1, primer día, en minutos: apertura 540, cierre 1020, tramos ordenados `[600, 690)`, `[630, 720)`, `[780, 840)`.

| tramo | hueco anterior | free_from después |
| --- | --- | --- |
| 600-690 | 540-600 (60): mejor | 690 |
| 630-720 | ninguno (630 < 690) | 720 |
| 780-840 | 720-780 (60): no es más largo | 840 |
| cierre | 840-1020 (180): mejor | |

La respuesta es 840-1020, que se escribe `14:00-17:00`.

**Por qué es correcto:** al llegar a un tramo que empieza en `s`, ya se han visto todos los que empiezan antes, y entre ellos cubren todo lo ocupado desde `open` hasta `free_from`. Si `s > free_from`, los minutos `free_from .. s − 1` no los cubre ningún tramo anterior (todos terminan como mucho en `free_from`) ni ninguno posterior (empiezan en `s` o después): forman un hueco libre que no se puede alargar por ningún lado. Todos los huecos libres aparecen así, como el hueco antes de algún tramo o antes de `close`, y aparecen del más temprano al más tardío. Por eso, cambiar el mejor solo con un hueco **estrictamente** más largo conserva el más temprano entre los de igual longitud. Si ningún hueco es positivo, la sala está completa: escribe `BOOKED`.

**Complejidad:** O(k log k) por día para ordenar y O(k) para el barrido; O(Σk log k) en total.

**Otra forma correcta:** marcar cada minuto del día como ocupado o libre en un arreglo de 1440 y recorrerlo. También cabe en estos límites, pero su coste depende de la duración de los tramos y no de cuántos hay; al barrido no le importa la resolución del reloj.

**Escritura:** rellena con ceros las dos partes: `f"{m // 60:02d}:{m % 60:02d}"` (Python), `String(h).padStart(2, "0")` (TypeScript), `sprintf("%02d:%02d", ...)` (PHP), `format!("{:02}:{:02}", ...)` (Rust). Lee toda la entrada estándar de una vez y escribe las `T` líneas de una sola vez.
