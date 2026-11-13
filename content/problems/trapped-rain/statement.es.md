Un muro de jardín está hecho de columnas de piedra de anchura 1 colocadas una junto a otra; `columns[i]` es la altura de la columna `i`. Tras una tormenta, el agua queda atrapada en los huecos entre columnas: encima de cada columna, el agua sube hasta la **menor** de dos alturas, la de la columna más alta a su izquierda y la de la columna más alta a su derecha (el agua se escapa por los dos extremos del muro).

Devuelve el total de agua atrapada, en cuadrados unidad.

### Ejemplo 1

```
columns = [3, 0, 2, 0, 4]
Salida: 7
```

Encima de las columnas de altura 0, 2 y 0, el agua llega a la altura 3: 3 + 1 + 3.

### Ejemplo 2

```
columns = [1, 2, 3]
Salida: 0
```

### Ejemplo 3

```
columns = [5, 1, 1, 6, 0, 2]
Salida: 10
```

4 + 4 encima de los dos 1, y 2 encima del 0.

### Restricciones

- 1 ≤ columns.length ≤ 2·10⁵
- 0 ≤ columns[i] ≤ 10⁴
