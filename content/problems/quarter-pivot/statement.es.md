Un huerto anota cada factura en `sales`: el día de la venta y el importe en céntimos. Para el informe anual, la dueña quiere una línea por año, con lo recaudado ese año repartido en sus cuatro trimestres, uno al lado del otro.

Los trimestres son los del calendario: T1 va de enero a marzo, T2 de abril a junio, T3 de julio a septiembre y T4 de octubre a diciembre. Un trimestre sin ventas muestra 0, y un año sin ninguna venta no aparece.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `sales(id, sold_on, amount_cents)`: `id` es la clave primaria; `sold_on` es el día de la venta, escrito `AAAA-MM-DD`; `amount_cents` es el importe de la factura, en céntimos.

## Resultado

Devuelve una fila por cada año con al menos una venta, con cinco columnas:

| columna | valor |
| --- | --- |
| `year` | el año, como entero |
| `q1` | la suma de los importes de ese año vendidos en enero, febrero o marzo, o 0 |
| `q2` | lo mismo para abril, mayo y junio |
| `q3` | lo mismo para julio, agosto y septiembre |
| `q4` | lo mismo para octubre, noviembre y diciembre |

Ordena las filas por `year`.

### Ejemplo 1

`sales`:

| id | sold_on | amount_cents |
| --- | --- | --- |
| 1 | 2025-01-15 | 1200 |
| 2 | 2025-03-31 | 800 |
| 3 | 2025-04-01 | 500 |
| 4 | 2025-12-24 | 3000 |
| 5 | 2026-07-04 | 2200 |

Resultado:

| year | q1 | q2 | q3 | q4 |
| --- | --- | --- | --- | --- |
| 2025 | 2000 | 500 | 0 | 3000 |
| 2026 | 0 | 0 | 2200 | 0 |

En 2025, las ventas de enero y marzo forman el T1 (1200 + 800), la de abril es el T2 y la de diciembre el T4; en el T3 no se vendió nada. 2026 solo tiene una venta en julio, en el T3.

### Ejemplo 2

`sales` está vacía.

Resultado: vacío (ninguna fila).

## Restricciones

- 0 ≤ ventas ≤ 3000.
- Los ids son distintos, de 1 a 10⁶.
- Cada `sold_on` es una fecha válida de 2020 a 2026.
- 1 ≤ `amount_cents` ≤ 10⁶.
