## El error

La primera línea construye la sala:

```python
hall = [["."] * cols] * rows
```

`["."] * cols` crea una lista de fila, y `* rows` crea una lista con `rows` referencias a esa misma fila. En memoria solo hay una fila, así que `hall[r][c] = "X"` cambia la columna `c` de todas las filas a la vez. Con una sola fila no se nota nada, y por eso pasa las pruebas pequeñas.

Con `rows = 2`, `cols = 1` y `bookings = [[0, 0]]`, el código devuelve `["X", "X"]`, pero solo se reservó la fila 0, así que la respuesta es `["X", "."]`.

## La corrección

Crea una lista nueva para cada fila:

```python
hall = [["."] * cols for _ in range(rows)]
```

La versión en TypeScript tiene el mismo problema porque `fill` coloca el mismo objeto array en cada posición; construye las filas con `Array.from({ length: rows }, () => new Array<string>(cols).fill("."))`.

**Complejidad:** O(rows · cols + b) en tiempo para b reservas, y O(rows · cols) de espacio para el plano.
