Una liga de fútbol cinco guarda su tabla en `teams`: el nombre de cada equipo, su división y los puntos que lleva. Al final de la temporada, cada división publica su clasificación.

Dentro de una división, el equipo con más puntos ocupa el **puesto 1**. Los equipos con los mismos puntos comparten puesto, y el siguiente número de puesto sigue sin huecos: tres equipos con 30, 30 y 27 puntos ocupan los puestos 1, 1 y 2. Cada división se clasifica por separado.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `teams(id, name, division, points)`: `id` es la clave primaria; `name` es el nombre del equipo; `division` es la letra de la división; `points` son los puntos del equipo.

## Resultado

Devuelve una fila por equipo, con cuatro columnas:

| columna | valor |
| --- | --- |
| `division` | la división del equipo |
| `name` | el nombre del equipo |
| `points` | los puntos del equipo |
| `place` | el puesto del equipo dentro de su división |

Ordena las filas por `division`, luego por `place`, luego por `name` y luego por el `id` del equipo.

### Ejemplo 1

`teams`:

| id | name | division | points |
| --- | --- | --- | --- |
| 1 | Rovers | A | 30 |
| 2 | United | A | 27 |
| 3 | Albion | A | 30 |
| 4 | Town | A | 25 |
| 5 | City | B | 12 |

Resultado:

| division | name | points | place |
| --- | --- | --- | --- |
| A | Albion | 30 | 1 |
| A | Rovers | 30 | 1 |
| A | United | 27 | 2 |
| A | Town | 25 | 3 |
| B | City | 12 | 1 |

Albion y Rovers comparten el puesto 1 en la división A y se listan por nombre; United va después, en el puesto 2, no el 3. City está sola en la división B, así que ocupa el puesto 1.

### Ejemplo 2

`teams`:

| id | name | division | points |
| --- | --- | --- | --- |
| 6 | Rangers | C | 0 |

Resultado:

| division | name | points | place |
| --- | --- | --- | --- |
| C | Rangers | 0 | 1 |

## Restricciones

- 1 ≤ equipos ≤ 2000.
- Los ids son distintos, de 1 a 10⁶.
- Un nombre es una palabra con mayúscula inicial, de 3 a 12 letras; dos equipos pueden tener el mismo nombre.
- `division` es `A`, `B`, `C` o `D`.
- 0 ≤ `points` ≤ 300.
