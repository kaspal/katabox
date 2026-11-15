Una aplicación para aprender idiomas registra cada inicio de sesión en `logins`: quién entró y qué día. A los alumnos les gusta mantener una **racha**: una serie de días naturales consecutivos con al menos un inicio de sesión cada día.

La aplicación quiere la racha más larga de cada alumno hasta ahora. Varios inicios de sesión el mismo día cuentan como un solo día, y una racha continúa al cambiar de mes o de año: el 31 de diciembre seguido del 1 de enero son dos días seguidos.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema. Una consulta `WITH`, con tantas expresiones de tabla comunes como necesites, cuenta como una sola sentencia `SELECT`.

## Tablas

- `logins(id, username, day)`: `id` es la clave primaria; `username` es el alumno; `day` es la fecha del inicio de sesión, escrita `AAAA-MM-DD`. Un alumno puede tener varias filas el mismo día.

## Resultado

Devuelve una fila por cada `username` que aparece en `logins`, con dos columnas:

| columna | valor |
| --- | --- |
| `username` | el alumno |
| `longest` | el número de días de la racha más larga de ese alumno |

Ordena las filas por `longest`, de mayor a menor, y luego por `username`.

### Ejemplo 1

`logins`:

| id | username | day |
| --- | --- | --- |
| 1 | ana_r | 2026-01-01 |
| 2 | ana_r | 2026-01-02 |
| 3 | ana_r | 2026-01-02 |
| 4 | ana_r | 2026-01-03 |
| 5 | ana_r | 2026-01-05 |
| 6 | bo99 | 2026-01-01 |
| 7 | bo99 | 2026-01-03 |

Resultado:

| username | longest |
| --- | --- |
| ana_r | 3 |
| bo99 | 1 |

ana_r entra el 1, el 2 (dos veces) y el 3 de enero, una racha de 3 días, y luego falta el día 4. bo99 falta el 2 de enero, así que cada una de sus rachas dura un solo día.

### Ejemplo 2

`logins`:

| id | username | day |
| --- | --- | --- |
| 8 | cyrus | 2025-12-31 |
| 9 | cyrus | 2026-01-01 |

Resultado:

| username | longest |
| --- | --- |
| cyrus | 2 |

## Restricciones

- 0 ≤ inicios de sesión ≤ 3000. Si no hay ninguno, el resultado está vacío.
- Los ids son distintos, de 1 a 10⁶.
- Un `username` tiene de 3 a 12 caracteres entre `a`-`z`, `0`-`9` y `_`, y empieza por una letra.
- Cada `day` es una fecha válida de 2025 o 2026.
