Una pequeña aerolínea guarda sus aeropuertos en `airports`, sus vuelos programados en `flights` y cada plaza vendida en `bookings`. Un vuelo solo conoce los códigos de los aeropuertos de salida y de llegada; los nombres de las ciudades están en `airports`.

Para los resúmenes de viaje que envía, la aerolínea quiere **cada tramo del viaje de cada pasajero**: una fila por reserva, con el número de vuelo y las ciudades entre las que vuela, y los tramos de cada pasajero en el orden en que salen.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `airports(code, city)`: `code` es la clave primaria, un código de aeropuerto de tres letras; `city` es la ciudad a la que sirve.
- `flights(id, flight_no, from_code, to_code, departs)`: `id` es la clave primaria; `from_code` (aeropuerto de salida) y `to_code` (aeropuerto de llegada) hacen referencia a `airports(code)`; `departs` es la hora de salida escrita `AAAA-MM-DD HH:MM`.
- `bookings(id, passenger, flight_id)`: `id` es la clave primaria; `passenger` es el nombre del viajero; `flight_id` hace referencia a `flights(id)`.

## Resultado

Devuelve una fila por reserva, con cuatro columnas:

| columna | valor |
| --- | --- |
| `passenger` | el nombre del pasajero |
| `flight_no` | el número del vuelo |
| `from_city` | la ciudad del aeropuerto de salida |
| `to_city` | la ciudad del aeropuerto de llegada |

Ordena las filas por `passenger`, luego por la hora de salida del vuelo (la más temprana primero) y luego por el `id` de la reserva.

### Ejemplo 1

`airports`:

| code | city |
| --- | --- |
| LIM | Lima |
| CUZ | Cusco |
| UIO | Quito |

`flights`:

| id | flight_no | from_code | to_code | departs |
| --- | --- | --- | --- | --- |
| 1 | KB101 | LIM | CUZ | 2026-06-01 08:00 |
| 2 | KB102 | CUZ | LIM | 2026-06-05 18:30 |
| 3 | KB200 | LIM | UIO | 2026-06-02 07:15 |

`bookings`:

| id | passenger | flight_id |
| --- | --- | --- |
| 1 | bo | 3 |
| 2 | ana | 2 |
| 3 | ana | 1 |

Resultado:

| passenger | flight_no | from_city | to_city |
| --- | --- | --- | --- |
| ana | KB101 | Lima | Cusco |
| ana | KB102 | Cusco | Lima |
| bo | KB200 | Lima | Quito |

ana reservó primero el vuelo de vuelta, pero KB101 sale el 1 de junio y KB102 el 5 de junio, así que su tramo de ida va antes. Los dos aeropuertos de cada vuelo dan dos ciudades distintas.

### Ejemplo 2

`airports`:

| code | city |
| --- | --- |
| LIM | Lima |
| CUZ | Cusco |

`flights`:

| id | flight_no | from_code | to_code | departs |
| --- | --- | --- | --- | --- |
| 4 | KB300 | CUZ | LIM | 2026-07-01 12:00 |

`bookings`: vacía.

Resultado: vacío (ninguna fila). Un vuelo que nadie reservó no da ninguna fila.

## Restricciones

- 2 ≤ aeropuertos ≤ 30, 1 ≤ vuelos ≤ 300, 0 ≤ reservas ≤ 2000.
- Los códigos de aeropuerto son distintos, tres letras mayúsculas; una ciudad es una palabra con mayúscula inicial.
- Los ids de vuelo y de reserva son distintos, de 1 a 10⁶. Un número de vuelo es `KB` seguido de tres dígitos.
- Un vuelo va entre dos aeropuertos existentes y distintos. Las horas de salida son distintas y son horas válidas de 2026.
- El nombre de un pasajero tiene de 1 a 10 letras minúsculas; cada reserva es de un vuelo existente. Un pasajero puede reservar el mismo vuelo más de una vez.
