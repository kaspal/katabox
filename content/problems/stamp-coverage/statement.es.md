Una niña juega con sellos rectangulares sobre una hoja cuadriculada de `rows` filas y `cols` columnas de casillas. El sello `[top, left, bottom, right]` entinta cada casilla de la fila `top` a la fila `bottom` y de la columna `left` a la columna `right`, todas incluidas. Una casilla puede entintarse muchas veces.

Devuelve `[most, cells]`: el mayor número de veces que se entintó una casilla, y cuántas casillas se entintaron exactamente ese número de veces.

### Ejemplo 1

```
rows = 3, cols = 4, stamps = [[0, 0, 1, 1], [1, 1, 2, 3], [0, 1, 2, 1]]
Salida: [3, 1]
```

Las veces que se entintó cada casilla son:

```
1 2 0 0
1 3 1 1
0 2 1 1
```

### Ejemplo 2

```
rows = 2, cols = 2, stamps = [[0, 0, 1, 1], [0, 0, 1, 1]]
Salida: [2, 4]
```

### Ejemplo 3

```
rows = 1, cols = 5, stamps = [[0, 0, 0, 1], [0, 3, 0, 4]]
Salida: [1, 4]
```

### Restricciones

- 1 ≤ rows, cols ≤ 500
- 1 ≤ stamps.length ≤ 10⁵
- 0 ≤ top ≤ bottom < rows, 0 ≤ left ≤ right < cols
