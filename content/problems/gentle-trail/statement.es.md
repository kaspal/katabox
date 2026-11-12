Un club de senderismo planea una ruta por una ladera representada como una cuadrícula: `heights[r][c]` es la altura del punto de la fila `r`, columna `c`, en centímetros. La ruta empieza en el punto de arriba a la izquierda, termina en el de abajo a la derecha y avanza de punto en punto **arriba, abajo, izquierda o derecha**. Puede dar todas las vueltas que quiera.

Un paso entre dos puntos vecinos es tan duro como la diferencia de sus alturas. Al club solo le importa el **paso más duro** de la ruta, no su longitud. Devuelve el menor valor posible del paso más duro entre todas las rutas.

### Ejemplo 1

```
heights = [[10, 30, 31], [12, 50, 33], [15, 17, 40]]
Salida: 20
```

Bajando por la izquierda, 10 → 12 → 15 → 17 → 40, el último paso es de 23. Por arriba, 10 → 30 → 31 → 33 → 40, el paso más duro es el primero, de 20. Ninguna ruta lo mejora.

### Ejemplo 2

```
heights = [[5]]
Salida: 0
```

La ruta empieza donde termina.

### Ejemplo 3

```
heights = [[1, 2, 3]]
Salida: 1
```

### Restricciones

- 1 ≤ heights.length ≤ 200
- 1 ≤ heights[i].length ≤ 200, todas las filas de la misma longitud
- 0 ≤ heights[i][j] ≤ 10⁶
