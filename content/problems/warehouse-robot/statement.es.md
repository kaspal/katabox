Un robot de reparto se mueve por el plano de un almacén, dado como `grid`: `0` es suelo libre y `1` es una estantería. El robot sale de la celda superior izquierda y debe llegar a la celda inferior derecha. Cada movimiento va a la celda inmediatamente de arriba, abajo, izquierda o derecha, y debe quedar en suelo libre dentro de la cuadrícula.

Devuelve el menor número de movimientos para llegar a la celda inferior derecha, o `-1` si no se puede llegar (también cuando la salida o la meta es una estantería).

### Ejemplo 1

```
grid = [[0, 0, 0],
        [1, 1, 0],
        [0, 0, 0]]
Salida: 4
```

Derecha, derecha, abajo, abajo.

### Ejemplo 2

```
grid = [[0, 1],
        [1, 0]]
Salida: -1
```

### Ejemplo 3

```
grid = [[0]]
Salida: 0
```

### Restricciones

- 1 ≤ rows, cols ≤ 200 (rows = grid.length, cols = grid[r].length en cada fila)
- grid[r][c] vale 0 o 1
