## Idea

Contar es fácil cuando cada paso por una estación es su propia fila:

1. **Despivotar** los viajes: una fila `(id del viaje, estación)` para la salida y otra para la llegada, fundiendo en una las dos filas idénticas de un viaje circular.
2. **Agrupar** esas filas por estación y contarlas.
3. **Filtrar** los grupos con 3 viajes o más, y ordenarlos.

## La consulta

```sql
SELECT station, COUNT(*) AS trips
FROM (
  SELECT id, start_station AS station FROM trips
  UNION
  SELECT id, end_station FROM trips
)
GROUP BY station
HAVING COUNT(*) >= 3
ORDER BY trips DESC, station;
```

## Detalles que importan

- **`UNION`, no `UNION ALL`.** `UNION ALL` deja las dos mitades tal cual, así que un viaje circular suma 2 a su estación. En el ejemplo 2, Bridge tendría entonces 4 y aparecería, aunque solo 2 viajes pasaron por ella. `UNION` quita las filas repetidas, y un viaje circular es justo el caso en que la fila de salida y la de llegada son idénticas.
- **Conserva el `id`.** El `id` es lo que hace correcta la eliminación de repetidos. Sin él, `SELECT start_station UNION SELECT end_station` fundiría todos los pasos por una estación en una sola fila, y cada estación contaría 1.
- **`HAVING`, no `WHERE`.** El recuento solo existe después de agrupar, así que la condición "al menos 3" va en `HAVING`. `ORDER BY` puede usar el alias `trips`.
- **Otra forma.** El `COUNT(*)` sobre la unión es, por estación, las salidas más las llegadas que no son viajes circulares: `UNION ALL` de `start_station` con `end_station` de los viajes donde `start_station <> end_station`. Las dos dan las mismas filas.
- **`minutes` es ruido.** La duración del viaje no interviene; está ahí porque las tablas reales traen columnas que una consulta no necesita.

## Coste

La unión ordena o indexa unas `2n` filas para quitar repetidos, y la agrupación las ordena por estación: más o menos `O(n log n)` para `n` viajes. La ordenación final solo ve las estaciones que pasaron el `HAVING`.
