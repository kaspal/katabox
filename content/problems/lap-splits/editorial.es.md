## Enfoque: trabajar en segundos y dar formato al final

Un tiempo como `4:05` tiene dos unidades, y sumar tiempos así a mano obliga a llevar los segundos a los minutos. Convertir antes cada vuelta a **segundos** elimina el problema: el total pasa a ser una suma normal y la vuelta más rápida un mínimo normal.

```
segundos("4:05") = 4 · 60 + 5 = 245
```

Para Ana, en el Ejemplo 1, las vueltas son 245, 238 y 252 segundos: el total es 735 y la más rápida 238. De vuelta a `m:ss`:

```
735 / 60 = 12 (división entera)   735 % 60 = 15    →  12:15
238 / 60 = 3                      238 % 60 = 58    →  3:58
```

Los minutos se escriben tal cual, sin tope en 59 (`119:58` en el Ejemplo 2), y los segundos siempre con dos cifras (`0:10`, no `0:1`).

**Por qué es correcto:** `m · 60 + ss` da a cada tiempo su duración exacta en segundos, así que sumar y comparar esos números es sumar y comparar los tiempos reales. La división entre 60 y el resto deshacen la conversión, de modo que el texto escrito es el tiempo correcto en el formato pedido.

**Una trampa:** comparar directamente las cadenas de las vueltas es tentador pero incorrecto: como texto, `"10:00" < "9:59"`, porque `1` va antes que `9`. Compara siempre números.

**Complejidad:** tiempo O(tamaño de la entrada); cada corredor solo necesita una suma y un mínimo acumulados.

**Lectura y escritura:** lee toda la entrada de una vez, divídela en tokens y recórrelos con una posición (nombre, `k` y luego `k` vueltas), como en Ticket Tally; junta las líneas de salida y escríbelas de una vez. El relleno en cada lenguaje: `f"{s // 60}:{s % 60:02d}"` (Python), `` `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}` `` (TypeScript; `/` sola da decimales), `sprintf("%d:%02d", intdiv($s, 60), $s % 60)` (PHP), `format!("{}:{:02}", s / 60, s % 60)` (Rust).
