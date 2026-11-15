Un club de inversión sigue a unas cuantas empresas cotizadas. La tabla `prices` guarda un precio de cierre por empresa (su `ticker`) y día de cotización, en céntimos. Los mercados cierran los fines de semana y los festivos, y el club no anotó todos los días, así que los días de cotización de una empresa no siempre son días naturales consecutivos, y dos empresas pueden cotizar en días distintos.

Para cada empresa y cada uno de sus días de cotización **salvo el primero**, el club quiere el cierre de ese día junto al **cierre anterior**: el cierre de la misma empresa en su día anterior más reciente de la tabla, con la variación en porcentaje.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `prices(id, ticker, day, close_cents)`: `id` es la clave primaria; `day` es una fecha escrita `AAAA-MM-DD`; `close_cents` es el precio de cierre en céntimos. Hay como mucho una fila por `ticker` y `day`, y las filas no siguen ningún orden.

## Resultado

Devuelve una fila por empresa y día de cotización que tenga un día anterior, con cinco columnas:

| columna | valor |
| --- | --- |
| `ticker` | la empresa |
| `day` | el día de cotización |
| `close_cents` | el cierre de `day` |
| `prev_cents` | el cierre de la misma empresa en su día de cotización anterior |
| `change_pct` | `100 × (close_cents − prev_cents) / prev_cents`, redondeado a 2 decimales |

Calcula `change_pct` con división real (no entera) y redondéalo con `ROUND(x, 2)` de SQLite, que redondea las mitades alejándose de cero. Se acepta un valor a menos de 0,001 del esperado.

Ordena las filas por `ticker` y luego por `day`.

### Ejemplo 1

`prices`:

| id | ticker | day | close_cents |
| --- | --- | --- | --- |
| 1 | ACME | 2026-03-02 | 1000 |
| 2 | ACME | 2026-03-03 | 1050 |
| 3 | ACME | 2026-03-04 | 1029 |
| 4 | BOLT | 2026-03-02 | 400 |
| 5 | BOLT | 2026-03-04 | 300 |

Resultado:

| ticker | day | close_cents | prev_cents | change_pct |
| --- | --- | --- | --- | --- |
| ACME | 2026-03-03 | 1050 | 1000 | 5.0 |
| ACME | 2026-03-04 | 1029 | 1050 | -2.0 |
| BOLT | 2026-03-04 | 300 | 400 | -25.0 |

ACME pasa de 1000 a 1050 (+5 %) y luego a 1029 (−2 %). BOLT no tiene fila para 2026-03-03, así que su cierre del 2026-03-04 se compara con el del 2026-03-02: de 400 a 300 es un −25 %. El primer día de cada empresa no tiene cierre anterior y no aparece.

### Ejemplo 2

`prices`:

| id | ticker | day | close_cents |
| --- | --- | --- | --- |
| 6 | CORA | 2026-03-02 | 777 |

Resultado: vacío (ninguna fila). CORA solo tiene un día, así que no tiene cierre anterior.

## Restricciones

- 0 ≤ filas en `prices` ≤ 3000.
- Los ids son distintos, de 1 a 10⁶.
- Un ticker son 3 o 4 letras mayúsculas.
- Cada `day` es una fecha válida de 2026, con como mucho una fila por ticker y día.
- 1 ≤ `close_cents` ≤ 10⁷.
