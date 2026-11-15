Un hospital guarda a todo su personal en una tabla, `staff`. Todos menos la directora dependen de exactamente una persona, cuyo `id` está en `boss_id`; el `boss_id` de la directora es `NULL`.

Para un nuevo organigrama, el hospital quiere saber a qué altura de la cadena de mando está cada persona. Su **profundidad** es el número de personas que tiene por encima: su jefe, el jefe de su jefe y así hasta la directora, incluida. La directora tiene profundidad 0, quienes dependen de ella tienen profundidad 1, y así sucesivamente.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema. Una consulta `WITH RECURSIVE` cuenta como una sola sentencia `SELECT`.

## Tablas

- `staff(id, name, boss_id)`: `id` es la clave primaria; `boss_id` es el `id` de la persona de la que depende esta, o `NULL` para la directora.

Exactamente una persona, la directora, tiene `boss_id` a `NULL`. El esquema no declara ninguna clave foránea, pero cualquier otro `boss_id` es el `id` de otro miembro del personal, y no hay ciclos: subiendo de jefe en jefe desde cualquiera se llega siempre a la directora. Una cadena de mando puede tener hasta 1000 personas.

## Resultado

Devuelve una fila por miembro del personal, con dos columnas:

| columna | valor |
| --- | --- |
| `name` | el nombre de la persona |
| `depth` | cuántas personas tiene por encima en la cadena de mando |

Ordena las filas por `depth`, de menor a mayor, luego por `name` y luego por el `id` de la persona.

### Ejemplo 1

`staff`:

| id | name | boss_id |
| --- | --- | --- |
| 1 | Iris | NULL |
| 2 | Bram | 1 |
| 3 | Cleo | 1 |
| 4 | Dev | 3 |
| 5 | Ada | 4 |

Resultado:

| name | depth |
| --- | --- |
| Iris | 0 |
| Bram | 1 |
| Cleo | 1 |
| Dev | 2 |
| Ada | 3 |

Iris es la directora. Bram y Cleo dependen de ella, Dev depende de Cleo y Ada de Dev, así que Ada tiene tres personas por encima: Dev, Cleo e Iris.

### Ejemplo 2

`staff`:

| id | name | boss_id |
| --- | --- | --- |
| 9 | Eva | NULL |

Resultado:

| name | depth |
| --- | --- |
| Eva | 0 |

## Restricciones

- 1 ≤ personal ≤ 1000.
- Los ids son distintos, de 1 a 10⁶.
- Un nombre es una palabra con mayúscula inicial de 2 a 12 letras; dos personas pueden llamarse igual.
- Hay exactamente una directora, y las cadenas de mando no tienen ciclos.
