Una furgoneta de reparto anota cuántos kilómetros recorre cada día: `miles[i]` para el día `i` (los días se cuentan desde 0). El contable pregunta por muchos **viajes**: cada viaje `[a, b]` abarca los días de `a` a `b`, ambos incluidos.

Devuelve los kilómetros totales de cada viaje, en el orden en que se preguntan.

### Ejemplo 1

```
miles = [4, 0, 7, 3, 5], trips = [[0, 2], [1, 3], [4, 4], [0, 4]]
Salida: [11, 10, 5, 19]
```

Los días 0 a 2 suman 4 + 0 + 7 = 11.

### Ejemplo 2

```
miles = [9], trips = [[0, 0], [0, 0]]
Salida: [9, 9]
```

### Ejemplo 3

```
miles = [2, 2, 2, 2], trips = [[1, 2]]
Salida: [4]
```

### Restricciones

- 1 ≤ miles.length ≤ 10⁵
- 0 ≤ miles[i] ≤ 1000
- 1 ≤ trips.length ≤ 10⁵
- 0 ≤ a ≤ b < miles.length para cada viaje [a, b]
