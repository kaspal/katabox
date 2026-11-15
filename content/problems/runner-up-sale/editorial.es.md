## Idea

"El segundo importe distinto más alto" es más fácil de calcular como "el importe más alto por debajo del máximo":

1. **Listar las regiones**, una vez cada una, para que todas tengan fila.
2. **Encontrar el máximo** de la región.
3. **Tomar el mayor importe estrictamente por debajo**; si no hay ninguno, la respuesta es `NULL` por sí sola.

## La consulta

```sql
SELECT r.region,
       (SELECT MAX(s.amount_cents)
        FROM sales s
        WHERE s.region = r.region
          AND s.amount_cents < (SELECT MAX(t.amount_cents) FROM sales t WHERE t.region = r.region)
       ) AS runner_up_cents
FROM (SELECT DISTINCT region FROM sales) r
ORDER BY r.region;
```

## Detalles que importan

- **La segunda fila no es la segunda.** `ORDER BY s.amount_cents DESC LIMIT 1 OFFSET 1` toma la segunda *venta*. Cuando dos ventas empatan en el máximo, esa segunda venta tiene el importe máximo, así que el máximo acaba siendo su propia segunda: South en el ejemplo 1 tendría 3000 en vez de 1000. Comparar con `<` contra el máximo salta todas sus copias. (`SELECT DISTINCT amount_cents ... LIMIT 1 OFFSET 1` también funcionaría.)
- **`NULL` sale gratis.** Un agregado como `MAX` sobre ninguna fila devuelve una fila con `NULL`, así que una región con un solo importe distinto obtiene `NULL` sin `CASE`. Una subconsulta con `LIMIT`/`OFFSET` que no encuentra fila también da `NULL`, pero un filtro como `WHERE amount_cents < máximo` en la consulta exterior eliminaría la región.
- **Todas las regiones aparecen.** Agrupar solo las ventas por debajo del máximo perdería regiones como East en el ejemplo 2. Partir de `SELECT DISTINCT region` las conserva.
- **Con funciones de ventana.** `DENSE_RANK() OVER (PARTITION BY region ORDER BY amount_cents DESC)` numera los importes distintos 1, 2, ... en cada región, y los empates comparten número. La segunda es entonces `MAX(CASE WHEN rnk = 2 THEN amount_cents END)` agrupando por región, que es `NULL` donde ningún importe tiene el puesto 2.

## Coste

Para cada una de las `r` regiones, las dos subconsultas recorren las ventas: unos `O(r · n)` pasos para `n` ventas, poco aquí porque hay pocas regiones. Un índice en `(region, amount_cents)` permitiría encontrar cada `MAX` en unos `O(log n)` pasos.
