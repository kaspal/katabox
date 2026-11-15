Una naviera tiene barcos entre los muelles de un gran lago. Cada fila de `routes` es una travesía directa y **de un solo sentido** de `from_pier` a `to_pier`; la vuelta, si existe, es otra fila. Las rutas pueden formar ciclos, incluso ciclos que vuelven al punto de partida.

Un billete de día comprado en el muelle llamado `Central` sirve para un máximo de **3 travesías**. La naviera quiere imprimir todos los demás muelles a los que un pasajero puede llegar desde `Central` con ese billete, junto con el menor número de travesías necesarias para llegar.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema. Una consulta `WITH RECURSIVE` cuenta como una sola sentencia `SELECT`.

## Tablas

- `routes(id, from_pier, to_pier)`: `id` es la clave primaria; cada fila es una travesía de `from_pier` a `to_pier`, solo en ese sentido.

Los muelles solo existen como nombres en `routes`. El muelle de partida se llama exactamente `Central`; puede no tener ninguna ruta.

## Resultado

Devuelve una fila por cada muelle, distinto de `Central`, al que se puede llegar desde `Central` en 3 travesías como mucho, con dos columnas:

| columna | valor |
| --- | --- |
| `pier` | el nombre del muelle |
| `hops` | el menor número de travesías necesarias para llegar desde `Central` (de 1 a 3) |

`Central` nunca aparece en la lista, aunque alguna ruta vuelva a él. Ordena las filas por `hops`, de menor a mayor, y luego por `pier`.

### Ejemplo 1

`routes`:

| id | from_pier | to_pier |
| --- | --- | --- |
| 1 | Central | Harbour |
| 2 | Harbour | Island |
| 3 | Island | Lighthouse |
| 4 | Lighthouse | Cove |
| 5 | Central | Island |
| 6 | Island | Central |

Resultado:

| pier | hops |
| --- | --- |
| Harbour | 1 |
| Island | 1 |
| Lighthouse | 2 |
| Cove | 3 |

Harbour e Island están a una travesía de Central; a Island también se llega pasando por Harbour en dos, pero cuenta el menor número de travesías. Lighthouse está a dos travesías pasando por Island, y Cove a tres. La ruta de Island de vuelta a Central no mete a Central en la lista.

### Ejemplo 2

`routes`:

| id | from_pier | to_pier |
| --- | --- | --- |
| 7 | Market | Central |

Resultado: vacío (ninguna fila). La única ruta llega a Central y ninguna sale de él.

## Restricciones

- 1 ≤ rutas ≤ 400.
- Los ids son distintos, de 1 a 10⁶.
- El nombre de un muelle es una palabra con mayúscula inicial de 3 a 15 letras.
- Una ruta une dos muelles distintos, y cada par de un sentido (`from_pier`, `to_pier`) aparece como mucho una vez.
