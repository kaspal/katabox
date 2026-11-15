A fin de mes, una tienda compara sus libros de cuentas con el banco. El extracto del banco está en `bank` y los registros propios de la tienda en `books`; los dos listan los pagos por su referencia de pago, con el importe en céntimos.

La contable quiere cada referencia en la que los dos **no cuadran**: un pago que el banco muestra pero que falta en los libros, uno de los libros que nunca llegó al banco, o uno que está en ambos con distinto importe. Las referencias que coinciden en los dos lados, con el mismo importe, están bien y no salen en la lista.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `bank(id, ref, amount_cents)`: `id` es la clave primaria; `ref` es la referencia del pago; `amount_cents` es el importe que registró el banco, en céntimos.
- `books(id, ref, amount_cents)`: las mismas columnas, tal como las registró la tienda.

Una referencia aparece como mucho una vez en cada tabla. Los ids de las dos tablas no tienen relación: empareja las filas por `ref`.

## Resultado

Devuelve una fila por cada referencia que no cuadra, con tres columnas:

| columna | valor |
| --- | --- |
| `ref` | la referencia del pago |
| `bank_cents` | el importe en `bank`, o `NULL` si la referencia no está ahí |
| `books_cents` | el importe en `books`, o `NULL` si la referencia no está ahí |

Ordena las filas por `ref`.

### Ejemplo 1

`bank`:

| id | ref | amount_cents |
| --- | --- | --- |
| 1 | P0001 | 1500 |
| 2 | P0002 | 2000 |
| 3 | P0004 | 700 |

`books`:

| id | ref | amount_cents |
| --- | --- | --- |
| 1 | P0001 | 1500 |
| 2 | P0002 | 2500 |
| 3 | P0003 | 900 |

Resultado:

| ref | bank_cents | books_cents |
| --- | --- | --- |
| P0002 | 2000 | 2500 |
| P0003 | NULL | 900 |
| P0004 | 700 | NULL |

P0001 cuadra y se queda fuera. P0002 está en ambos con importes distintos, P0003 solo en los libros y P0004 solo en el banco.

### Ejemplo 2

`bank`:

| id | ref | amount_cents |
| --- | --- | --- |
| 1 | P0100 | 4200 |

`books`:

| id | ref | amount_cents |
| --- | --- | --- |
| 9 | P0100 | 4200 |

Resultado: vacío (ninguna fila). El único pago cuadra, aunque sus ids sean distintos.

## Restricciones

- 0 ≤ filas en `bank` ≤ 2000 y 0 ≤ filas en `books` ≤ 2000, con al menos una fila en total. Cualquiera de las dos tablas puede estar vacía.
- Dentro de cada tabla, los ids son distintos, de 1 a 10⁶, y las referencias son distintas; una referencia es `P` seguida de cuatro dígitos.
- −10⁷ ≤ `amount_cents` ≤ 10⁷; un importe puede ser cero o negativo (una devolución).
- SQLite 3.53 admite `FULL OUTER JOIN`, pero se acepta cualquier `SELECT` correcta.
