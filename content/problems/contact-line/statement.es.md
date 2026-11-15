Un taller de reparaciones llama o escribe a sus clientes cuando una reparación está lista. Cada cliente puede tener un teléfono, un correo, los dos o ninguno, y puede haber dicho por qué canal prefiere que lo contacten.

En recepción quieren una lista impresa con una línea por cliente, escrita como `last_name, first_name: contacto`. El contacto se elige así:

1. Si el cliente prefiere un canal (`prefers` vale `'phone'` o `'email'`) y el valor de ese canal no es `NULL`, se usa ese.
2. Si no, se usa el teléfono si no es `NULL` y, en su defecto, el correo. Es decir, sin preferencia, o cuando el canal preferido es `NULL`, el teléfono va primero.
3. Si el teléfono y el correo son los dos `NULL`, el contacto es el texto `no contact`.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `clients(id, first_name, last_name, phone, email, prefers)`: `id` es la clave primaria; `phone` y `email` son el teléfono y el correo del cliente, cada uno `NULL` si no se conoce; `prefers` vale `'phone'`, `'email'` o `NULL` cuando el cliente no tiene preferencia.

## Resultado

Devuelve una fila por cliente, con dos columnas:

| columna | valor |
| --- | --- |
| `id` | el `id` del cliente |
| `contact_line` | `last_name`, luego `, `, luego `first_name`, luego `: ` y luego el contacto |

Ordena las filas por `last_name`, luego por `first_name` y luego por `id`.

### Ejemplo 1

`clients`:

| id | first_name | last_name | phone | email | prefers |
| --- | --- | --- | --- | --- | --- |
| 1 | Ana | Rios | 555-0101 | ana@rios.test | email |
| 2 | Bruno | Soto | 555-0102 | NULL | email |
| 3 | Clara | Vega | NULL | clara@vega.test | NULL |
| 4 | Dario | Mora | NULL | NULL | phone |

Resultado:

| id | contact_line |
| --- | --- |
| 4 | Mora, Dario: no contact |
| 1 | Rios, Ana: ana@rios.test |
| 2 | Soto, Bruno: 555-0102 |
| 3 | Vega, Clara: clara@vega.test |

Ana prefiere el correo, así que su correo gana a su teléfono. Bruno prefiere el correo pero no tiene, así que se usa su teléfono; Clara no tiene preferencia ni teléfono, así que se usa su correo; Dario no tiene ninguno de los dos.

### Ejemplo 2

`clients`:

| id | first_name | last_name | phone | email | prefers |
| --- | --- | --- | --- | --- | --- |
| 8 | Elena | Paz | 555-0199 | elena@paz.test | NULL |

Resultado:

| id | contact_line |
| --- | --- |
| 8 | Paz, Elena: 555-0199 |

## Restricciones

- 1 ≤ clientes ≤ 300; los ids son distintos, de 1 a 10⁶.
- `first_name` y `last_name` son cada uno una palabra con mayúscula inicial (`Ana`, `Rios`); varios clientes pueden compartir los dos.
- Un teléfono es `NULL` o tiene la forma `ddd-dddd` (`555-0101`); un correo es `NULL` o `nombre@dominio.test` en minúsculas.
- `prefers` vale `'phone'`, `'email'` o `NULL`.
