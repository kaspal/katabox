## Idea

Tres pasos, todos sobre la misma tabla:

1. **Limpiar** cada nombre: `trim` quita los espacios de alrededor y `lower` pasa todo a minúsculas, así cada forma de escribir una ciudad se convierte en un único valor.
2. **Ordenar la pareja**: el nombre limpio menor va en `city_a` y el mayor en `city_b`, de modo que los dos sentidos de una ruta dan la misma fila.
3. **Quitar los duplicados** con `DISTINCT`, tras descartar los vuelos cuyas dos ciudades son la misma.

## La consulta

```sql
SELECT DISTINCT
       min(lower(trim(origin)), lower(trim(dest))) AS city_a,
       max(lower(trim(origin)), lower(trim(dest))) AS city_b
FROM flights
WHERE lower(trim(origin)) <> lower(trim(dest))
ORDER BY city_a, city_b;
```

## Detalles que importan

- **Una fila por ruta, no por sentido.** Limpiar los nombres y seleccionar `DISTINCT origin, dest` sigue devolviendo `lima, quito` y `quito, lima` como dos filas. Primero hay que poner la pareja en un orden fijo, y `min` y `max` con dos argumentos hacen justo eso.
- **`min` y `max` con dos argumentos no son de agregado.** Con un solo argumento resumen toda la tabla en una fila; con dos o más trabajan fila a fila y devuelven el argumento menor o el mayor. Por eso la consulta no necesita `GROUP BY`.
- **Limpiar antes de cada comparación.** El filtro también debe comparar los nombres limpios: `origin <> dest` conserva `'Cali'` a `'cali'` y `'Arica'` a `'  Arica'`, que son vuelos de una ciudad a sí misma. Si se olvida `trim`, `' cusco'` pasa a ser otra ciudad distinta de `'cusco'`, y como el espacio va antes que cualquier letra, además queda mal ordenada.
- **Minúsculas en el resultado.** `DISTINCT` y la ordenación trabajan sobre los valores que devuelves, así que la salida deben ser los nombres limpios en minúsculas, no la grafía original de alguno de los vuelos.
- **Sin `NULL`.** Las dos columnas son `NOT NULL`, así que ninguna ruta se pierde por una ciudad desconocida.

## Coste

Una pasada por los vuelos para limpiar los nombres; después SQLite quita los duplicados y ordena las rutas con un árbol B temporal: unos n log n pasos para n vuelos.
