## Idea

Tres pasos, cada uno una cláusula del `SELECT`:

1. **Unir** cada préstamo con su socio, para tener el nombre.
2. **Filtrar** los préstamos tardíos: devueltos estrictamente después de la fecha prevista.
3. **Calcular y ordenar** los días de retraso.

## La consulta

```sql
SELECT m.name, l.title,
       CAST(julianday(l.returned) - julianday(l.due) AS INTEGER) AS days_late
FROM loans l
JOIN members m ON m.id = l.member_id
WHERE l.returned > l.due
ORDER BY days_late DESC, l.title, l.id;
```

## Detalles que importan

- **A tiempo no es tarde.** Con `>=` aparecerían los libros devueltos el día previsto con `days_late = 0`. La condición es estrictamente `>`.
- **`NULL` se filtra solo.** Una comparación con `NULL` no es ni verdadera ni falsa, así que `WHERE l.returned > l.due` descarta los libros aún prestados sin añadir `IS NOT NULL`.
- **Fechas como texto.** Las cadenas `AAAA-MM-DD` se ordenan como las fechas, así que compararlas como texto es seguro. Para *restar* fechas hacen falta números: `julianday` da el número de día, y la diferencia entre dos días completos es un número entero, que `CAST(... AS INTEGER)` convierte en entero.
- **Un orden completo.** Dos préstamos pueden tener el mismo retraso y el mismo título (el mismo libro prestado dos veces), así que el `id` del préstamo deshace el último empate. `ORDER BY` puede usar el alias `days_late` y columnas que no están en el resultado.

## Coste

Con un índice en `members.id` (la clave primaria), SQLite busca el socio de cada préstamo directamente: más o menos un paso por préstamo y luego ordenar las filas tardías.
