Un museo expone un mosaico como una cuadrícula de brillos de baldosas, `grid`, con `r` filas y `c` columnas. Esta noche los conservadores lo cuelgan girado un cuarto de vuelta **en el sentido de las agujas del reloj**: la columna izquierda pasa a ser la fila de arriba (leída de abajo arriba) y la fila de arriba pasa a ser la columna derecha.

Devuelve la cuadrícula girada. Tiene `c` filas y `r` columnas.

### Ejemplo 1

```
grid = [[1, 2, 3],
        [4, 5, 6]]
Salida: [[4, 1],
         [5, 2],
         [6, 3]]
```

### Ejemplo 2

```
grid = [[7], [8], [9]]
Salida: [[9, 8, 7]]
```

Una sola columna se convierte en una sola fila.

### Ejemplo 3

```
grid = [[42]]
Salida: [[42]]
```

### Restricciones

- 1 ≤ r, c ≤ 300, donde r = grid.length y c = grid[i].length (todas las filas miden lo mismo)
- 0 ≤ grid[i][j] ≤ 255
