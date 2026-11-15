Una tienda de artículos para el hogar prepara un estante de «regalos de 20 a 50». Todos los productos están en `products`, con el precio guardado en céntimos para que nunca se cuele un redondeo, y con una marca que indica si ahora mismo están disponibles.

La tienda quiere los productos para ese estante: **disponibles** (`in_stock = 1`) y con un precio **de 2000 a 4999 céntimos, ambos extremos incluidos** (de 20,00 a 49,99). Un producto que cuesta exactamente 20,00 o exactamente 49,99 va al estante; uno de 50,00, no.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `products(id, name, price_cents, in_stock)`: `id` es la clave primaria; `price_cents` es el precio en céntimos (`2599` significa 25,99); `in_stock` vale `1` cuando el producto está disponible y `0` cuando no.

## Resultado

Devuelve una fila por cada producto del estante, con dos columnas:

| columna | valor |
| --- | --- |
| `name` | el nombre del producto |
| `price_cents` | su precio en céntimos |

Ordena las filas por `price_cents`, de mayor a menor, luego por `name` y luego por el `id` del producto.

### Ejemplo 1

`products`:

| id | name | price_cents | in_stock |
| --- | --- | --- | --- |
| 1 | lamp | 2000 | 1 |
| 2 | rug | 4999 | 1 |
| 3 | vase | 5000 | 1 |
| 4 | mug | 1999 | 1 |
| 5 | clock | 3500 | 0 |
| 6 | tray | 3500 | 1 |

Resultado:

| name | price_cents |
| --- | --- |
| rug | 4999 |
| tray | 3500 |
| lamp | 2000 |

Los productos lamp y rug están justo en los dos extremos de la franja, así que entran los dos. En cambio, vase y mug se quedan a un céntimo de ella, y clock no está disponible.

### Ejemplo 2

`products`:

| id | name | price_cents | in_stock |
| --- | --- | --- | --- |
| 1 | pen | 2500 | 1 |
| 2 | ink | 2500 | 1 |
| 3 | pad | 2500 | 1 |

Resultado:

| name | price_cents |
| --- | --- |
| ink | 2500 |
| pad | 2500 |
| pen | 2500 |

## Restricciones

- 1 ≤ productos ≤ 500; los ids son distintos, de 1 a 10⁶.
- Un nombre es una o dos palabras en minúsculas; dos productos pueden llamarse igual.
- 1 ≤ `price_cents` ≤ 10⁶, un número entero.
- `in_stock` vale `0` o `1`.
