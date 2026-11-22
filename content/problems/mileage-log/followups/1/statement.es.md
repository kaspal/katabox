Esta vez el contable lleva **toda la flota**. `miles[v][d]` es cuántos kilómetros recorrió
la furgoneta `v` el día `d` (furgonetas y días se cuentan desde 0, y cada furgoneta tiene
un dato para cada día). Cada pregunta `[v1, d1, v2, d2]` pide los kilómetros totales de
las furgonetas `v1` a `v2` durante los días `d1` a `d2`, con los cuatro límites incluidos:
un rectángulo de la cuadrícula.

Devuelve el total de cada pregunta, en el orden en que se preguntan.

### Ejemplo 1

```
miles = [[10, 0, 5], [7, 3, 2]], queries = [[0, 0, 1, 2], [1, 1, 1, 2], [0, 2, 1, 2]]
Salida: [27, 5, 7]
```

La última pregunta abarca el día 2 de las dos furgonetas: 5 + 2 = 7.

### Ejemplo 2

```
miles = [[4, 18, 2, 8], [3, 15, 14, 15]], queries = [[0, 0, 1, 3], [0, 0, 1, 3], [1, 0, 1, 1]]
Salida: [79, 79, 18]
```

### Ejemplo 3

```
miles = [[1, 2, 2, 11], [5, 9, 8, 19]], queries = [[0, 1, 0, 3], [1, 2, 1, 3], [0, 2, 0, 3]]
Salida: [15, 27, 13]
```

### Restricciones

- 1 ≤ furgonetas, días y furgonetas × días ≤ 10⁵ (todas las furgonetas tienen el mismo número de días)
- 0 ≤ miles[v][d] ≤ 1000
- 1 ≤ queries.length ≤ 10⁵
- 0 ≤ v1 ≤ v2 < furgonetas y 0 ≤ d1 ≤ d2 < días para cada pregunta [v1, d1, v2, d2]
