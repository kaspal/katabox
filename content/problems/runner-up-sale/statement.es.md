Una cadena de muebles registra cada venta en `sales`, con la región donde se hizo y el importe en céntimos. Cada trimestre premia la venta más grande de cada región, y con un premio menor a la **segunda**.

La segunda de una región es su segundo importe *distinto* más alto: el mayor importe que queda estrictamente por debajo del importe máximo de la región. Dos ventas empatadas en el máximo son las dos el máximo, así que ninguna es la segunda. Una región cuyas ventas tienen todas el mismo importe no tiene segunda.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `sales(id, region, amount_cents)`: `id` es la clave primaria; `region` es el nombre de la región; `amount_cents` es el importe de la venta, en céntimos.

## Resultado

Devuelve una fila por cada región que tenga al menos una venta, con dos columnas:

| columna | valor |
| --- | --- |
| `region` | el nombre de la región |
| `runner_up_cents` | el segundo importe distinto más alto de la región, o `NULL` si tiene menos de dos importes distintos |

Ordena las filas por `region`.

### Ejemplo 1

`sales`:

| id | region | amount_cents |
| --- | --- | --- |
| 1 | North | 5000 |
| 2 | North | 9000 |
| 3 | North | 7000 |
| 4 | South | 3000 |
| 5 | South | 3000 |
| 6 | South | 1000 |

Resultado:

| region | runner_up_cents |
| --- | --- |
| North | 7000 |
| South | 1000 |

El importe máximo de North es 9000, así que su segunda es 7000. En South dos ventas comparten el máximo, 3000, así que la segunda es el siguiente importe, 1000.

### Ejemplo 2

`sales`:

| id | region | amount_cents |
| --- | --- | --- |
| 7 | East | 2500 |
| 8 | East | 2500 |

Resultado:

| region | runner_up_cents |
| --- | --- |
| East | NULL |

East tiene dos ventas pero un solo importe distinto, así que no tiene segunda; la región aparece igualmente.

## Restricciones

- 1 ≤ ventas ≤ 2000.
- Los ids son distintos, de 1 a 10⁶.
- Una región es una palabra con mayúscula inicial, de 3 a 12 letras.
- 1 ≤ `amount_cents` ≤ 10⁷.
