Una papelería registra cada línea de cada venta en `sales`: la categoría del producto y el importe, en céntimos.

El dueño quiere ver de dónde sale el dinero: para cada categoría, el total de sus ventas y su **cuota** sobre todas las ventas de la tienda, en porcentaje. Las categorías más grandes van primero.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `sales(id, category, amount_cents)`: `id` es la clave primaria; `category` es la categoría del producto; `amount_cents` es el importe de la línea en céntimos.

## Resultado

Devuelve una fila por categoría, con tres columnas:

| columna | valor |
| --- | --- |
| `category` | la categoría |
| `total_cents` | la suma de `amount_cents` de sus líneas |
| `share_pct` | 100 × `total_cents` ÷ la suma de todos los `amount_cents` de la tabla, calculado de forma exacta (con decimales) y luego redondeado a 1 decimal con el `ROUND` de SQLite |

Ordena las filas por `share_pct` (el valor redondeado), de mayor a menor, y luego por `category`. Se aceptan valores de `share_pct` a menos de 0.001 de los esperados.

### Ejemplo 1

`sales`:

| id | category | amount_cents |
| --- | --- | --- |
| 1 | pens | 300 |
| 2 | paper | 500 |
| 3 | pens | 200 |
| 4 | ink | 1000 |

Resultado:

| category | total_cents | share_pct |
| --- | --- | --- |
| ink | 1000 | 50.0 |
| paper | 500 | 25.0 |
| pens | 500 | 25.0 |

Todas las ventas suman 2000 céntimos. Ink aporta 1000, así que un 50.0 %; paper y pens aportan 500 cada una, un 25.0 %, y se ordenan por nombre.

### Ejemplo 2

`sales`:

| id | category | amount_cents |
| --- | --- | --- |
| 5 | cards | 999 |

Resultado:

| category | total_cents | share_pct |
| --- | --- | --- |
| cards | 999 | 100.0 |

## Restricciones

- 1 ≤ ventas ≤ 3000.
- Los ids son distintos, de 1 a 10⁶.
- Una categoría tiene de 3 a 12 letras minúsculas.
- 1 ≤ `amount_cents` ≤ 10⁶.
