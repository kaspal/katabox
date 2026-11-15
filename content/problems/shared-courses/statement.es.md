Una escuela de idiomas guarda a sus alumnos en `students` y cada matrícula en `enrollments`: cada fila dice que un alumno cursa un curso. Un alumno puede cursar varios, y cada curso tiene muchos alumnos.

Para formar grupos de estudio, la escuela quiere todas las **parejas de alumnos que comparten al menos un curso**. Cada pareja aparece una sola vez, aunque los dos compartan varios cursos, y nadie forma pareja consigo mismo.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `students(id, name)`: `id` es la clave primaria; no hay dos alumnos con el mismo nombre.
- `enrollments(id, student_id, course)`: `student_id` hace referencia a `students(id)`; `course` es un código de curso como `ENG101`. Un alumno se matricula en cada curso como mucho una vez.

## Resultado

Devuelve una fila por pareja, con dos columnas:

| columna | valor |
| --- | --- |
| `student_a` | el nombre del alumno con el `id` **menor** |
| `student_b` | el nombre del alumno con el `id` mayor |

Ordena las filas por el `id` de `student_a` y luego por el `id` de `student_b` (los ids no salen en el resultado).

### Ejemplo 1

`students`:

| id | name |
| --- | --- |
| 1 | ana |
| 2 | bo |
| 3 | cy |

`enrollments`:

| id | student_id | course |
| --- | --- | --- |
| 1 | 1 | ENG101 |
| 2 | 2 | ENG101 |
| 3 | 1 | FRA101 |
| 4 | 2 | FRA101 |
| 5 | 3 | GER101 |

Resultado:

| student_a | student_b |
| --- | --- |
| ana | bo |

ana y bo comparten dos cursos, ENG101 y FRA101, pero son una sola pareja, así que salen una vez. cy está solo en GER101.

### Ejemplo 2

`students`:

| id | name |
| --- | --- |
| 4 | di |
| 5 | ed |

`enrollments`:

| id | student_id | course |
| --- | --- | --- |
| 6 | 4 | ITA101 |
| 7 | 5 | JPN101 |

Resultado: vacío (ninguna fila). di y ed están en cursos distintos.

## Restricciones

- 1 ≤ alumnos ≤ 150, 0 ≤ matrículas ≤ 600.
- Los ids de alumno son distintos, de 1 a 10⁶; los nombres son distintos, de 1 a 10 letras minúsculas.
- Un código de curso son tres letras mayúsculas y tres dígitos.
- Cada matrícula pertenece a un alumno existente, y un alumno se matricula en cada curso como mucho una vez. Un alumno puede no tener matrículas.
