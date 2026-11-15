## Idea

Un filtro de dos partes y una ordenación:

1. **Quedarse** con los productos disponibles.
2. **Quedarse** con los precios dentro de la franja cerrada 2000..4999.
3. **Ordenar** del más caro al más barato, con el nombre y luego el id para deshacer empates.

## La consulta

```sql
SELECT name, price_cents
FROM products
WHERE in_stock = 1 AND price_cents BETWEEN 2000 AND 4999
ORDER BY price_cents DESC, name, id;
```

## Detalles que importan

- **Los extremos son parte de la franja.** `price_cents > 2000 AND price_cents < 4999` parece natural, pero deja fuera los productos de exactamente 20,00 y exactamente 49,99, que la tienda sí quiere. `BETWEEN a AND b` significa `>= a AND <= b`, así que los dos extremos entran.
- **Los céntimos lo hacen exacto.** En céntimos la franja va de 2000 a 4999, no de 2000 a 5000: 49,99 es el último precio que entra y 50,00 queda fuera. Como los precios son enteros, no hay redondeos de los que preocuparse.
- **Filtrar por columnas que no se devuelven.** `in_stock` e `id` no están en el resultado, pero `WHERE` y `ORDER BY` pueden usarlas igualmente.
- **Un orden completo.** Dos productos pueden tener el mismo nombre y el mismo precio, así que el `id` deshace el último empate, aunque no se muestre.

## Coste

Una pasada por los productos para comprobar las dos condiciones y luego ordenar las filas que quedan. Un índice sobre `price_cents` permitiría a SQLite leer solo la franja, pero con este tamaño un recorrido completo sirve igual.
