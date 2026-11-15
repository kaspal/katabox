## Idea

Tres pasos, cada uno una cláusula del `SELECT`:

1. **Unir** cada pedido con el cliente que lo hizo, a través de `orders.customer_id`.
2. **Filtrar** los pedidos grandes: `total_cents` de 10000 o más.
3. **Renombrar y ordenar**: el `id` del pedido pasa a llamarse `order_id`, y las filas van del importe mayor al menor.

## La consulta

```sql
SELECT o.id AS order_id, c.name, c.city, o.total_cents
FROM orders o
JOIN customers c ON c.id = o.customer_id
WHERE o.total_cents >= 10000
ORDER BY o.total_cents DESC, o.id;
```

## Detalles que importan

- **Unir por la clave foránea.** `ON c.id = o.id` parece razonable, porque las dos columnas se llaman `id`, pero empareja el pedido 1 con el cliente 1 solo porque los números coinciden. En el Ejemplo 1 daría el pedido 1 a Ana en lugar de a Bruno, y un pedido cuyo id no fuera id de ningún cliente desaparecería. Lo que une las tablas es `o.customer_id`.
- **"Al menos" es `>=`.** Un pedido de 100,00 justos (`10000`) es grande; con `>` se perdería. El pedido 2 del Ejemplo 1, de `9999`, muestra el otro lado del límite.
- **Céntimos, no unidades.** `total_cents >= 100` dejaría pasar casi todos los pedidos. El umbral es 100,00 expresado en céntimos.
- **Un orden completo.** Dos pedidos pueden tener el mismo importe, incluso del mismo cliente, así que `o.id` deshace el empate. Ordenar por nombre no bastaría, porque los nombres se repiten.
- **Alias.** Las dos tablas tienen `id`, así que cada columna debe ir cualificada (`o.id`, `c.name`), y la columna del resultado debe llamarse `order_id`.

## Coste

Con la clave primaria en `customers.id`, SQLite busca directamente el cliente de cada pedido grande: más o menos un paso por pedido y luego ordenar los pedidos grandes.
