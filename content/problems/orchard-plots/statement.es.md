Un huerto está plantado como una cuadrícula de árboles; `grid[r][c]` es cuántas cajas de manzanas dio este año el árbol de la fila `r`, columna `c`. Los compradores preguntan por parcelas rectangulares: la consulta `[top, left, bottom, right]` abarca las filas de `top` a `bottom` y las columnas de `left` a `right`, todas incluidas.

Devuelve el total de cajas de cada parcela, en el orden de las consultas.

### Ejemplo 1

```
grid = [[3, 0, 2],
        [1, 4, 5],
        [6, 2, 1]],
queries = [[0, 0, 1, 1], [1, 1, 2, 2], [0, 2, 2, 2]]
Salida: [8, 12, 8]
```

La primera parcela es 3 + 0 + 1 + 4; la tercera es la columna de la derecha, 2 + 5 + 1.

### Ejemplo 2

```
grid = [[7]], queries = [[0, 0, 0, 0]]
Salida: [7]
```

### Ejemplo 3

```
grid = [[1, 2, 3, 4]], queries = [[0, 1, 0, 3], [0, 0, 0, 0]]
Salida: [9, 1]
```

### Restricciones

- 1 ≤ filas, columnas ≤ 300 (todas las filas tienen la misma longitud)
- 0 ≤ grid[r][c] ≤ 1000
- 1 ≤ queries.length ≤ 10⁵
- 0 ≤ top ≤ bottom < filas, 0 ≤ left ≤ right < columnas
