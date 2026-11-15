Una panadería de barrio guarda a sus clientes en `customers` y cada pedido en `orders`. Los importes se guardan en céntimos, así que un pedido de 100,00 tiene `total_cents = 10000`.

Para las tartas grandes, la dueña quiere llamar a todos los que hicieron un **pedido grande**: un pedido de al menos 100,00. Necesita cada uno de esos pedidos con el nombre y la ciudad del cliente.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `customers(id, name, city)`: `id` es la clave primaria.
- `orders(id, customer_id, placed, total_cents)`: `id` es la clave primaria; `customer_id` hace referencia a `customers(id)`; `placed` es la fecha del pedido, escrita `AAAA-MM-DD`; `total_cents` es el importe del pedido en céntimos.

Los ids de clientes y los de pedidos son numeraciones independientes: un pedido y un cliente pueden tener el mismo `id` sin tener nada que ver.

## Resultado

Devuelve una fila por cada pedido con `total_cents` de 10000 o más, con cuatro columnas:

| columna | valor |
| --- | --- |
| `order_id` | el `id` del pedido |
| `name` | el nombre del cliente que lo hizo |
| `city` | la ciudad de ese cliente |
| `total_cents` | el importe del pedido en céntimos |

Ordena las filas por `total_cents`, de mayor a menor, y luego por `order_id`.

### Ejemplo 1

`customers`:

| id | name | city |
| --- | --- | --- |
| 1 | Ana | Porto |
| 2 | Bruno | Braga |
| 3 | Clara | Faro |

`orders`:

| id | customer_id | placed | total_cents |
| --- | --- | --- | --- |
| 1 | 2 | 2026-03-01 | 12500 |
| 2 | 3 | 2026-03-01 | 9999 |
| 3 | 1 | 2026-03-02 | 10000 |
| 4 | 2 | 2026-03-03 | 2500 |

Resultado:

| order_id | name | city | total_cents |
| --- | --- | --- | --- |
| 1 | Bruno | Braga | 12500 |
| 3 | Ana | Porto | 10000 |

El pedido 1 es del cliente 2 (Bruno), no del cliente 1. El pedido 3 es justo de 100,00, así que cuenta; al pedido 2 le falta un céntimo.

### Ejemplo 2

`customers`:

| id | name | city |
| --- | --- | --- |
| 5 | Elena | Evora |

`orders`:

| id | customer_id | placed | total_cents |
| --- | --- | --- | --- |
| 7 | 5 | 2026-04-10 | 800 |

Resultado: ninguna fila (solo las columnas `order_id`, `name`, `city` y `total_cents`).

## Restricciones

- 1 ≤ clientes ≤ 200, 0 ≤ pedidos ≤ 2000.
- Los ids de clientes son distintos entre sí, y también los de pedidos, cada uno de 1 a 10⁶.
- El `customer_id` de cada pedido es el id de un cliente.
- Un nombre o una ciudad es una sola palabra con mayúscula inicial; dos clientes pueden llamarse igual.
- `placed` es una fecha válida de 2026; 1 ≤ `total_cents` ≤ 10⁷.
