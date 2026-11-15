## Enfoque: las dos agujas en medios grados

Mide cada aguja en el sentido de las agujas del reloj desde el 12. La esfera tiene 12 horas, así que primero pliega la hora de 24 horas: `h = hh mod 12` (las 13:00 y la 01:00 se ven igual, y las 12:00 son 0).

- El minutero gira 360 grados en 60 minutos: `6 · mm` grados.
- La aguja horaria gira 360 grados en 12 horas, 30 grados por hora, y avanza medio grado cada minuto: `30 · h + 0.5 · mm` grados.

La única fracción es ese medio grado. **Duplicarlo todo** la elimina: en medios grados la aguja horaria está en `60 · h + mm` y el minutero en `12 · mm`, y una vuelta completa son 720. Así

```
gap       = |60 · h − 11 · mm|        (0 ≤ gap ≤ 660)
respuesta = min(gap, 720 − gap)       medios grados, 0..360
```

y el ángulo es `respuesta / 2` grados. Siempre es un número entero o un entero y medio, así que escribe `respuesta / 2` con división entera y después `.5` si `respuesta` es impar y `.0` si es par.

Para las 09:45: `h = 9`, `gap = |540 − 495| = 45`, `min(45, 675) = 45` medios grados, que se escribe `22.5`. Para las 00:59: `gap = |0 − 649| = 649`, y por el otro lado es más corto: `720 − 649 = 71`, que se escribe `35.5`.

**Por qué es correcto:** las dos fórmulas son las posiciones exactas de las agujas, y la separación entre dos direcciones en un círculo es su diferencia o el resto del círculo, la que sea menor. Trabajar en medios grados solo cambia la unidad, así que no se redondea nada en ningún momento.

**Con coma flotante también funciona:** un múltiplo de 0.5 se guarda exacto en un double, y el formato con un decimal (`toFixed(1)`, `sprintf("%.1f")`, `{:.1}`) lo escribe exacto. La versión con enteros simplemente no necesita ese razonamiento. No olvides el `mod 12`: sin él, las 18:00 darían `|1080 − 0| = 1080` medios grados y un «otro lado» negativo.

**Complejidad:** O(1) por hora, O(Q) en total.

**Lectura y escritura:** lee toda la entrada de una vez, divídela en tokens y saca las horas y los minutos de cada `hh:mm` (los caracteres antes y después de los dos puntos; `"09"` se lee como 9 en todos los lenguajes). Junta las líneas de salida y escríbelas de una vez.
