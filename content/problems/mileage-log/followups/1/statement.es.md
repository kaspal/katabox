El registro de la furgoneta es el mismo: `miles[i]` kilómetros el día `i` (los días se
cuentan desde 0). Esta vez el contable revisa las normas de descanso de los conductores y
pregunta por los **días largos**: cada pregunta `[a, b, x]` pide cuántos días de `a` a `b`
(los dos incluidos) la furgoneta recorrió **al menos** `x` kilómetros.

Devuelve la respuesta de cada pregunta, en el orden en que se hicieron.

### Ejemplo 1

```
miles = [5, 12, 0, 30, 7], trips = [[0, 4, 7], [1, 3, 13], [2, 2, 0]]
Salida: [3, 1, 1]
```

Los días 1, 3 y 4 recorrieron al menos 7 km; entre los días 1 y 3 solo el día 3 llegó a
13 km; el día 2 recorrió 0 km, que es al menos 0.

### Ejemplo 2

```
miles = [100, 100, 100], trips = [[0, 2, 100], [0, 2, 101], [1, 1, 50]]
Salida: [3, 0, 1]
```

### Restricciones

- 1 ≤ miles.length ≤ 10⁵
- 0 ≤ miles[i] ≤ 1000
- 1 ≤ trips.length ≤ 10⁵
- `trips[j] = [a, b, x]` con 0 ≤ a ≤ b < miles.length y 0 ≤ x ≤ 1000
