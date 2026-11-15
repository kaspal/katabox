Un pequeño boletín guarda a sus lectores en una tabla. Cuando alguien se apunta en una feria, el voluntario del puesto escribe lo que le dan: a veces una dirección correcta, a veces nada y a veces solo unos cuantos toques en la barra espaciadora.

Antes del próximo número, la editora quiere llamar a todos los que **no tienen un correo utilizable**. Un correo falta cuando es `NULL`, cuando es la cadena vacía o cuando está formado solo por espacios.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `readers(id, name, email, city)`: `id` es la clave primaria; `name` es el nombre del lector; `email` es lo que se escribió como dirección, o `NULL` si no se anotó nada; `city` es la ciudad donde vive.

## Resultado

Devuelve una fila por cada lector al que le falta el correo, con tres columnas:

| columna | valor |
| --- | --- |
| `id` | el `id` del lector |
| `name` | el nombre del lector |
| `city` | la ciudad del lector |

Ordena las filas por `city`, luego por `name` y luego por `id`.

### Ejemplo 1

`readers`:

| id | name | email | city |
| --- | --- | --- | --- |
| 1 | ana | ana@mail.test | Lima |
| 2 | bo | NULL | Quito |
| 3 | cy | `''` (vacía) | Lima |
| 4 | di | `'   '` (tres espacios) | Bogota |
| 5 | ed | ed@mail.test | Quito |

Resultado:

| id | name | city |
| --- | --- | --- |
| 4 | di | Bogota |
| 3 | cy | Lima |
| 2 | bo | Quito |

El lector bo no tiene correo, el de cy es la cadena vacía y el de di son tres espacios, así que a los tres les falta; salen ordenados por ciudad: Bogota, Lima, Quito.

### Ejemplo 2

`readers`:

| id | name | email | city |
| --- | --- | --- | --- |
| 7 | fay | fay@mail.test | Cusco |

Resultado: ninguna fila (las columnas siguen siendo `id`, `name`, `city`).

## Restricciones

- 1 ≤ lectores ≤ 300; los ids son distintos, de 1 a 10⁶.
- Un nombre tiene de 1 a 12 letras minúsculas; una ciudad es una palabra con mayúscula inicial (`Lima`, `Quito`).
- Un correo es `NULL` o tiene hasta 30 caracteres, cada uno una letra minúscula, `@`, `.` o un espacio. Los espacios son los únicos caracteres en blanco que aparecen.
- Dos lectores pueden tener el mismo nombre y la misma ciudad.
