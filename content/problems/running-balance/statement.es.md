Una cooperativa de crédito guarda cada movimiento en `transactions`: la cuenta, el día y el importe en céntimos, positivo para un ingreso y negativo para una retirada. Toda cuenta empieza con saldo 0.

Para los extractos de los socios, el empleado quiere cada movimiento junto al **saldo de la cuenta justo después de él**: la suma de los importes de esa cuenta hasta él, incluido. Los movimientos se aplican por orden de día; cuando una cuenta tiene varios el mismo día, va primero el de menor `id`.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `transactions(id, account, day, amount_cents)`: `id` es la clave primaria; `account` es el código de la cuenta; `day` se escribe `AAAA-MM-DD`; `amount_cents` es el importe en céntimos, positivo para ingresos y negativo para retiradas.

## Resultado

Devuelve una fila por movimiento, con cuatro columnas:

| columna | valor |
| --- | --- |
| `account` | el código de la cuenta |
| `day` | el día del movimiento |
| `amount_cents` | el importe del movimiento |
| `balance_cents` | el saldo de la cuenta después de este movimiento |

Ordena las filas por `account`, luego por `day` y luego por el `id` del movimiento.

### Ejemplo 1

`transactions`:

| id | account | day | amount_cents |
| --- | --- | --- | --- |
| 1 | AC001 | 2026-02-01 | 10000 |
| 2 | AC001 | 2026-02-03 | -2500 |
| 3 | AC002 | 2026-02-01 | 500 |
| 4 | AC001 | 2026-02-03 | -1000 |
| 5 | AC002 | 2026-02-02 | 700 |

Resultado:

| account | day | amount_cents | balance_cents |
| --- | --- | --- | --- |
| AC001 | 2026-02-01 | 10000 | 10000 |
| AC001 | 2026-02-03 | -2500 | 7500 |
| AC001 | 2026-02-03 | -1000 | 6500 |
| AC002 | 2026-02-01 | 500 | 500 |
| AC002 | 2026-02-02 | 700 | 1200 |

AC001 tiene dos retiradas el 2026-02-03: el movimiento 2 va primero y deja 7500, y luego el movimiento 4 deja 6500. AC002 lleva su propio saldo, desde 0.

### Ejemplo 2

`transactions` está vacía.

Resultado: vacío (ninguna fila).

## Restricciones

- 0 ≤ movimientos ≤ 3000.
- Los ids son distintos, de 1 a 10⁶; no tienen por qué seguir el orden de los días.
- Una cuenta es `AC` seguida de tres dígitos.
- Cada `day` es una fecha válida de 2026.
- `amount_cents` no es cero y −10⁶ ≤ `amount_cents` ≤ 10⁶; un saldo puede quedar por debajo de cero.
