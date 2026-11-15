## Idea

El cierre anterior es sencillamente la fila de antes, una vez puestas por fecha las filas de cada empresa. Una función de ventana lo lee directamente:

1. **Separar** las filas por `ticker` y **ordenar** las de cada empresa por `day`.
2. **Mirar una fila atrás** con `LAG`: el cierre del día de cotización anterior, o `NULL` en el primer día.
3. **Quitar** los primeros días, **calcular** la variación y **ordenar** el resultado.

## La consulta

```sql
SELECT ticker, day, close_cents, prev_cents,
       ROUND(100.0 * (close_cents - prev_cents) / prev_cents, 2) AS change_pct
FROM (
  SELECT ticker, day, close_cents,
         LAG(close_cents) OVER (PARTITION BY ticker ORDER BY day) AS prev_cents
  FROM prices
)
WHERE prev_cents IS NOT NULL
ORDER BY ticker, day;
```

## Detalles que importan

- **`PARTITION BY ticker`.** Sin él, `LAG(close_cents) OVER (ORDER BY ticker, day)` recorre una sola lista larga con todas las empresas. Dentro de una empresa da lo mismo, pero el primer día de cada empresa después de la primera recibe el último cierre de la *empresa anterior* en lugar de `NULL`, así que no se filtra. En el ejemplo 1 aparecería el 2026-03-02 de BOLT, comparado con el 1029 de ACME, con un −61,13 %. La partición reinicia la mirada atrás para cada ticker.
- **Día de cotización anterior, no ayer.** Unir cada fila con la de `date(day, '-1 day')` pierde todos los días después de un hueco, como el 2026-03-04 de BOLT en el ejemplo 1, y todos los lunes. `LAG` toma la fila que haya antes por orden de fecha, esté lo lejos que esté.
- **Filtrar después de la ventana.** `WHERE` se ejecuta antes que las funciones de ventana, así que `prev_cents` no se puede comprobar en el mismo `SELECT` que lo calcula. Filtrar antes además quitaría justo las filas que `LAG` necesita mirar. La subconsulta calcula la ventana sobre todas las filas y la consulta exterior filtra.
- **División real.** `close_cents` y `prev_cents` son enteros, y SQLite divide enteros como enteros: `100 * (8 - 7) / 7` da 14, no 14,29. Escribir `100.0` hace real toda la expresión. Otras formas, como dividir primero y multiplicar por 100 después, pueden diferir en las últimas cifras de coma flotante; las respuestas esperadas usan exactamente la expresión de arriba, redondeada con `ROUND`.
- **Sin división por cero.** Todo cierre es de al menos 1 céntimo, así que `prev_cents` nunca es 0.
- **Fechas como texto.** `AAAA-MM-DD` se ordena como las fechas, así que `ORDER BY day` es correcto, y el orden de los `id` de las filas no importa.

## Coste

La ventana necesita las filas ordenadas por `(ticker, day)`, y el orden final es el mismo, así que la consulta es una ordenación más una pasada: `O(n log n)`. Una autounión que busque, para cada fila, el último día anterior del mismo ticker también funciona, pero sin índice compara cada fila con todas las demás de su ticker.
