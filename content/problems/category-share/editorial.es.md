## Idea

Una cuota compara una parte con el todo, así que la consulta necesita las dos cosas a la vez:

1. **Agrupar** las ventas por categoría para obtener cada parte, `SUM(amount_cents)`.
2. **Obtener el todo** con una subconsulta escalar sobre toda la tabla.
3. **Dividir** como números reales, redondear y ordenar por la cuota redondeada.

## La consulta

```sql
SELECT category,
       SUM(amount_cents) AS total_cents,
       ROUND(100.0 * SUM(amount_cents) / (SELECT SUM(amount_cents) FROM sales), 1) AS share_pct
FROM sales
GROUP BY category
ORDER BY share_pct DESC, category;
```

## Detalles que importan

- **División entera.** Las dos sumas son enteros, y SQLite divide dos enteros como enteros, descartando el resto. Por eso `100 * SUM(amount_cents) / total` da un número entero antes de que `ROUND` lo vea: una categoría con 1 de 3 céntimos obtiene 33.0 en lugar de 33.3. Escribir `100.0` hace real el producto, así que la división conserva los decimales. Multiplicar primero también importa: `SUM(amount_cents) / total * 100` con enteros da 0 para todas las categorías salvo cuando hay una sola.
- **El total general.** Dentro de una consulta agrupada, `SUM(amount_cents)` es la suma del grupo. La subconsulta `(SELECT SUM(amount_cents) FROM sales)` no está correlacionada, así que SQLite la calcula una vez. Una función de ventana, `SUM(SUM(amount_cents)) OVER ()`, da el mismo total.
- **Ordena por la cuota redondeada.** El orden usa el alias, que es el valor redondeado, así que dos categorías cuyas cuotas exactas difieren un poco pero se redondean al mismo número se ordenan por `category`. Ordenar por la cuota sin redondear podría ponerlas al revés.
- **Sin división entre cero.** Cada importe es al menos 1 y la tabla tiene al menos una fila, así que el total es positivo.

## Coste

Una pasada de agrupación sobre las `n` líneas de venta, ordenadas por categoría, y otra para el total general: más o menos `O(n log n)`. La ordenación final solo ve una fila por categoría.
