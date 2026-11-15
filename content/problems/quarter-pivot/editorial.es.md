## Idea

Convertir filas en columnas ("pivotar") es agrupar y sumar con condiciones:

1. **Agrupar** las ventas por año: una fila de resultado por cada año con ventas.
2. **Sumar con condición**: la columna de cada trimestre suma el importe de una venta cuando su mes cae en ese trimestre, y 0 si no.
3. **Ordenar** por año.

## La consulta

```sql
SELECT CAST(substr(sold_on, 1, 4) AS INTEGER) AS year,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 1 AND 3 THEN amount_cents ELSE 0 END) AS q1,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 4 AND 6 THEN amount_cents ELSE 0 END) AS q2,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 7 AND 9 THEN amount_cents ELSE 0 END) AS q3,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 10 AND 12 THEN amount_cents ELSE 0 END) AS q4
FROM sales
GROUP BY year
ORDER BY year;
```

## Detalles que importan

- **La fórmula del trimestre.** Calcular el trimestre como `mes / 3 + 1` parece limpio, pero falla por uno al final de cada trimestre: marzo da 2, junio 3, septiembre 4 y diciembre 5, que no es ningún trimestre. En el ejemplo 1, la venta de marzo pasaría al T2 y la de diciembre desaparecería. La fórmula correcta es `(mes - 1) / 3 + 1` (división entera); los rangos explícitos con `BETWEEN` evitan la aritmética.
- **0, no `NULL`.** Sin `ELSE 0`, un `CASE` que no coincide con ninguna fila del grupo da `NULL` en todas, y el `SUM` de solo `NULL` es `NULL`. `ELSE 0` hace que un trimestre vacío valga 0. (`TOTAL` daría el decimal `0.0`, que no es el entero 0.)
- **El año como entero.** `substr` devuelve texto, así que sin la conversión `year` saldría como el texto `'2025'` en vez del entero `2025`. Lo mismo con el mes: `'03' BETWEEN 1 AND 3` compara texto con números y en SQLite es falso, así que conviértelo antes de comparar. (`strftime('%m', sold_on)` también devuelve texto.)
- **Años sin ventas.** Agrupar solo crea grupos para los años que aparecen, así que un año sin ventas no sale, como pide el enunciado; una tabla vacía no da filas.

## Coste

Una pasada por las `n` ventas, sumando cada importe a los totales de su año, y luego ordenar los pocos años: unos `O(n)` pasos más `O(y log y)` para `y` años.
