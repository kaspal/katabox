## Idea

Tres pasos, cada uno una cláusula del `SELECT`:

1. **Obtener el día** de cada pedido: los 10 primeros caracteres de `placed`.
2. **Agrupar** los pedidos por ese día, contándolos y sumando sus totales.
3. **Ordenar** los grupos por día.

## La consulta

```sql
SELECT substr(placed, 1, 10) AS day,
       COUNT(*) AS orders,
       SUM(total_cents) AS revenue_cents
FROM orders
GROUP BY day
ORDER BY day;
```

## Detalles que importan

- **Agrupa por el día, no por el momento.** `GROUP BY placed` crea un grupo por cada minuto distinto: en el ejemplo 1 los dos pedidos de las 12:05 comparten grupo, pero el de las 19:40 tiene el suyo, así que el 1 de mayo sale en dos filas (2 pedidos y luego 1). La columna `day` parece correcta en cada fila, lo que hace fácil pasar el error por alto; lo que decide las filas es la clave de agrupación.
- **El alias en `GROUP BY`.** SQLite permite que `GROUP BY day` use el alias de la lista del `SELECT`. `GROUP BY substr(placed, 1, 10)` es lo mismo y funciona en cualquier base de datos.
- **Las fechas como texto se ordenan como fechas.** Las cadenas `AAAA-MM-DD` se ordenan como el calendario, así que `ORDER BY day` no necesita conversión. `date(placed)` también daría el día, porque SQLite entiende `AAAA-MM-DD HH:MM` como un momento.
- **Solo días con pedidos.** Agrupar solo produce grupos que tienen filas, así que los días vacíos nunca aparecen y una tabla vacía da un resultado vacío. Listar todos los días de un rango necesitaría un calendario (por ejemplo, una CTE recursiva), y este problema no lo pide.

## Coste

SQLite ordena los pedidos por día (o crea un índice temporal sobre él) y luego los recorre una vez: más o menos `O(n log n)` para `n` pedidos. El resultado tiene una fila por día.
