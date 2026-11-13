Una red de estaciones meteorológicas a lo largo de un valle pronostica la temperatura nocturna de cada estación: `temps[i]` para la estación `i` (desde 0). Los meteorólogos revisan y preguntan sin parar, `ops[j]` con los números `args[j]`:

- `"shift"` con `[a, b, delta]`: cada pronóstico de la estación `a` a la `b` (ambas incluidas) cambia en `delta`, que puede ser negativo.
- `"coldest"` con `[a, b]`: informa del pronóstico actual más bajo entre las estaciones `a` a `b`.

Devuelve las respuestas de las operaciones `"coldest"`, en orden.

### Ejemplo 1

```
temps = [5, -2, 7, 0, 3], ops = ["coldest", "shift", "coldest", "coldest"], args = [[0, 4], [1, 3, 4], [1, 3], [2, 4]]
Salida: [-2, 2, 3]
```

Tras desplazar las estaciones 1 a 3 en +4, los pronósticos son [5, 2, 11, 4, 3].

### Ejemplo 2

```
temps = [1, 1, 1], ops = ["shift", "shift", "coldest"], args = [[0, 1, -5], [1, 2, -5], [0, 2]]
Salida: [-9]
```

La estación 1 se desplaza dos veces: 1 − 5 − 5 = −9.

### Ejemplo 3

```
temps = [8], ops = ["coldest"], args = [[0, 0]]
Salida: [8]
```

### Restricciones

- 1 ≤ temps.length ≤ 10⁵
- -1000 ≤ temps[i] ≤ 1000
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"shift"` lleva `[a, b, delta]` con 0 ≤ a ≤ b < temps.length y -50 ≤ delta ≤ 50; `"coldest"` lleva `[a, b]` con 0 ≤ a ≤ b < temps.length.
