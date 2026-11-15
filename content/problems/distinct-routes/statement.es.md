Una pequeña aerolínea chárter anota cada vuelo a mano en `flights`. Los empleados no son muy cuidadosos: la misma ciudad aparece como `'Lima'`, `'LIMA'` o `' lima '`, según quién la escribiera.

El equipo de planificación quiere la lista de **rutas distintas** que ha volado la aerolínea. Una ruta une dos ciudades diferentes, sin importar el sentido: un vuelo de Lima a Quito y otro de Quito a Lima son la misma ruta. Un vuelo que sale y llega a la misma ciudad no es una ruta.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `flights(id, origin, dest)`: `id` es la clave primaria; `origin` y `dest` son la ciudad de salida y la de llegada del vuelo, tal como se escribieron. Ninguna es nunca `NULL`.

Dos nombres son la misma ciudad cuando coinciden tras quitar los espacios de alrededor y sin distinguir mayúsculas de minúsculas: `' Cusco'`, `'cusco'` y `'CUSCO  '` son todos `cusco`.

## Resultado

Devuelve una fila por ruta, con dos columnas:

| columna | valor |
| --- | --- |
| `city_a` | la ciudad de la ruta que va primero en orden alfabético |
| `city_b` | la otra ciudad |

Escribe las dos ciudades en minúsculas y sin los espacios de alrededor, de modo que `city_a < city_b` en cada fila. Cada ruta aparece una sola vez, por muchos vuelos que la usaran y en el sentido que fuera. Ordena las filas por `city_a` y luego por `city_b`.

### Ejemplo 1

`flights` (las comillas muestran los espacios):

| id | origin | dest |
| --- | --- | --- |
| 1 | `'Lima'` | `'Quito'` |
| 2 | `'quito'` | `'LIMA'` |
| 3 | `' Cusco'` | `'Lima '` |
| 4 | `'Lima'` | `'Cusco'` |

Resultado:

| city_a | city_b |
| --- | --- |
| cusco | lima |
| lima | quito |

Los vuelos 1 y 2 son la ruta lima-quito, volada en los dos sentidos y escrita de forma distinta. Los vuelos 3 y 4 son ambos la ruta cusco-lima una vez quitados los espacios y las mayúsculas.

### Ejemplo 2

`flights`:

| id | origin | dest |
| --- | --- | --- |
| 5 | `'Cali'` | `'cali'` |
| 6 | `'Arica'` | `'  Arica'` |

Resultado: ninguna fila (solo las columnas `city_a` y `city_b`). Los dos vuelos salen y llegan a la misma ciudad.

## Restricciones

- 1 ≤ vuelos ≤ 2000; los ids son distintos.
- El nombre de una ciudad tiene de 2 a 12 letras (solo `A-Z`, `a-z`), con como mucho dos espacios delante y como mucho dos detrás.
