Esta vez el jardín no es un solo muro, sino todo un campo de columnas de piedra. `heights` es una cuadrícula: `heights[r][c]` es la altura de la columna de 1 × 1 de la fila `r`, columna `c`. Tras una tormenta, el agua se acumula encima de las columnas. Fluye entre columnas que comparten un lado (arriba, abajo, izquierda o derecha) y se escapa del jardín por encima de cualquier columna del borde exterior.

Así, el agua sobre una columna sube hasta el nivel más bajo al que podría escaparse: entre todos los caminos de columnas vecinas por un lado que van de ella a una columna del borde, toma aquel cuya columna más alta (contando los dos extremos) sea la más baja; esa altura es el nivel del agua, y la columna guarda el nivel menos su propia altura. Las columnas del borde nunca guardan agua. Devuelve el total de agua atrapada, en cubos unidad.

### Ejemplo 1

```
heights = [[3, 3, 3, 3], [3, 1, 2, 3], [3, 3, 3, 3]]
Salida: 3
```

Las dos columnas interiores están rodeadas por altura 3: 2 + 1.

### Ejemplo 2

```
heights = [[1, 4, 6, 6, 6], [0, 2, 0, 3, 6], [3, 3, 5, 3, 6], [1, 0, 3, 0, 6]]
Salida: 2
```

El 0 del centro de la fila 1 se llena hasta 2; si subiera más, el agua pasaría por encima de su vecina izquierda (altura 2) hasta el 0 del borde.

### Ejemplo 3

```
heights = [[6, 6, 0, 0, 0], [2, 6, 1, 5, 6], [5, 6, 2, 2, 4], [1, 4, 0, 4, 5]]
Salida: 0
```

Cada columna interior tiene un camino hasta el borde que nunca sube por encima de su propia altura.

### Restricciones

- 1 ≤ filas ≤ 200
- 1 ≤ columnas ≤ 200, todas las filas de la misma longitud
- 0 ≤ heights[r][c] ≤ 10⁴
