La tienda de la esquina quiere un pequeño gestor de existencias. Empieza vacío y recibe una lista de operaciones: `ops[i]` es el nombre de la operación y `args[i]` sus números. Los artículos se identifican por un código de producto.

- `"stock"`, args `[item, qty]`: llegan `qty` unidades más de `item`.
- `"sell"`, args `[item, qty]`: un cliente quiere `qty` unidades de `item`. La tienda vende todas las que tiene, hasta `qty`. **Informa** del número de unidades vendidas.
- `"count"`, args `[item]`: **informa** de cuántas unidades de `item` quedan.

Devuelve los informes en el orden en que ocurren las operaciones (`"stock"` no informa de nada).

### Ejemplo 1

```
ops = ["stock", "sell", "count", "sell", "count"], args = [[7, 5], [7, 3], [7], [7, 4], [7]]
Salida: [3, 2, 2, 0]
```

Llegan cinco unidades del artículo 7 y se venden 3; quedan 2. El siguiente cliente quiere 4, pero solo quedan 2, así que se venden 2 y no queda ninguna.

### Ejemplo 2

```
ops = ["count"], args = [[42]]
Salida: [0]
```

Un artículo que nunca llegó tiene 0 unidades.

### Ejemplo 3

```
ops = ["stock", "stock", "sell", "count", "count"], args = [[1, 10], [2, 4], [1, 10], [1], [2]]
Salida: [10, 0, 4]
```

### Restricciones

- 1 ≤ ops.length ≤ 10⁴, args.length = ops.length
- Cada `ops[i]` es `"stock"`, `"sell"` o `"count"`, con los números descritos arriba
- 1 ≤ item ≤ 10⁹, 1 ≤ qty ≤ 10⁴
