Una redacción asigna mentores a sus periodistas. Todo el personal está en una sola tabla, `staff`, y cada fila dice quién es el mentor de esa persona, si tiene. Un mentor no es más que otro miembro del personal.

La editora quiere la **lista de mentorías** completa: cada miembro del personal junto al nombre de su mentor. Quien no tiene mentor debe seguir en la lista, con el mentor vacío.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `staff(id, name, mentor_id)`: `id` es la clave primaria; `mentor_id` es el `id` del mentor de esta persona en la misma tabla, o `NULL` si no tiene. El esquema no declara `REFERENCES`, pero todo `mentor_id` que no sea `NULL` es el id de otro miembro del personal.

## Resultado

Devuelve una fila por miembro del personal, con dos columnas:

| columna | valor |
| --- | --- |
| `name` | el nombre del miembro del personal |
| `mentor` | el nombre de su mentor, o `NULL` si no tiene |

Ordena las filas por el `name` del miembro del personal y luego por su `id`.

### Ejemplo 1

`staff`:

| id | name | mentor_id |
| --- | --- | --- |
| 1 | Ada | NULL |
| 2 | Bea | 1 |
| 3 | Carl | 1 |
| 4 | Dina | 3 |

Resultado:

| name | mentor |
| --- | --- |
| Ada | NULL |
| Bea | Ada |
| Carl | Ada |
| Dina | Carl |

Ada no tiene mentor, así que su fila lleva `NULL`. Ada es mentora de Bea y de Carl, y Carl a su vez lo es de Dina.

### Ejemplo 2

`staff`:

| id | name | mentor_id |
| --- | --- | --- |
| 5 | Eli | NULL |

Resultado:

| name | mentor |
| --- | --- |
| Eli | NULL |

## Restricciones

- 1 ≤ personal ≤ 500; los ids son distintos, de 1 a 10⁶.
- Un nombre es una sola palabra con mayúscula inicial. Los nombres pueden repetirse: dos miembros del personal pueden llamarse igual, y entonces va primero el de `id` menor.
- `mentor_id` es `NULL` o el id de otro miembro del personal distinto. Una persona puede ser mentora de varias, y dos personas pueden ser mentoras la una de la otra.
