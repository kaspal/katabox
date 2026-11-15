El servicio de bicicletas compartidas de una ciudad registra cada viaje en `trips`: la estación donde se tomó la bici, la estación donde se dejó y cuánto duró el viaje.

El equipo de planificación quiere saber qué estaciones tienen más movimiento. Un viaje pasa por su estación de salida y por su estación de llegada, así que cuenta una vez para cada una. Un **viaje circular**, que sale y llega a la misma estación, solo pasa por una estación y cuenta **una vez** en ella. Las estaciones tranquilas, con menos de 3 viajes, quedan fuera.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `trips(id, start_station, end_station, minutes)`: `id` es la clave primaria; `start_station` y `end_station` son nombres de estación; `minutes` es la duración del viaje (aquí no hace falta).

## Resultado

Devuelve una fila por cada estación con al menos 3 viajes, con dos columnas:

| columna | valor |
| --- | --- |
| `station` | el nombre de la estación |
| `trips` | el número de viajes que salen o llegan a ella, con cada viaje circular contado una vez |

Ordena las filas por `trips`, de mayor a menor, y luego por `station`.

### Ejemplo 1

`trips`:

| id | start_station | end_station | minutes |
| --- | --- | --- | --- |
| 1 | Market | Park | 12 |
| 2 | Park | Market | 15 |
| 3 | Museum | Museum | 40 |
| 4 | Market | Museum | 9 |
| 5 | Museum | Park | 20 |

Resultado:

| station | trips |
| --- | --- |
| Market | 3 |
| Museum | 3 |
| Park | 3 |

Museum tiene los viajes 3, 4 y 5; el viaje 3 es circular y cuenta una vez. Market (viajes 1, 2 y 4) y Park (viajes 1, 2 y 5) también tienen 3, así que las tres estaciones empatan y se ordenan por nombre.

### Ejemplo 2

`trips`:

| id | start_station | end_station | minutes |
| --- | --- | --- | --- |
| 6 | Bridge | Bridge | 30 |
| 7 | Bridge | Bridge | 25 |

Resultado: vacío (ninguna fila). Bridge solo tiene 2 viajes.

## Restricciones

- 0 ≤ viajes ≤ 3000.
- Los ids son distintos, de 1 a 10⁶.
- El nombre de una estación es una palabra con mayúscula inicial, de 3 a 12 letras.
- 1 ≤ `minutes` ≤ 600.
