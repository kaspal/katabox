Un huerto comunitario está rodeado por una valla. La valla es un polígono simple (sus lados nunca se cruzan ni se tocan salvo en las esquinas compartidas), dado por sus esquinas `fence[i] = [x, y]` en orden alrededor del huerto, en sentido horario o antihorario.

Para cada punto `points[j] = [x, y]` donde un hortelano quiere plantar, devuelve exactamente una de estas cadenas:

- `"inside"` si el punto está estrictamente dentro del huerto,
- `"boundary"` si está sobre la valla (un lado o una esquina),
- `"outside"` en otro caso.

### Ejemplo 1

```
fence = [[0, 0], [6, 0], [6, 4], [3, 2], [0, 4]], points = [[1, 1], [3, 3], [6, 2], [3, 2], [7, 1]]
Salida: ["inside", "outside", "boundary", "boundary", "outside"]
```

La valla tiene una muesca en (3, 2), así que (3, 3) queda fuera.

### Ejemplo 2

```
fence = [[0, 0], [4, 0], [0, 4]], points = [[1, 1], [2, 2], [3, 2]]
Salida: ["inside", "boundary", "outside"]
```

### Ejemplo 3

```
fence = [[0, 0], [2, 0], [2, 2], [0, 2]], points = [[0, 0], [1, 1], [-1, 1]]
Salida: ["boundary", "inside", "outside"]
```

### Restricciones

- 3 ≤ fence.length ≤ 300, y la valla es un polígono simple con esquinas distintas.
- 1 ≤ points.length ≤ 1000
- Todas las coordenadas son enteros con valor absoluto de como mucho 10⁴.
