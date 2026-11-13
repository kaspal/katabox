Una fila de medidores de un río informa del nivel del agua: `readings[i]` para el medidor `i` (desde 0). Los ingenieros envían correcciones y preguntas sin parar, `ops[j]` con los números `args[j]`:

- `"add"` con `[a, b, delta]`: todos los medidores de `a` a `b` (ambos incluidos) estaban desviados en `delta`; suma `delta` a cada uno. `delta` puede ser negativo.
- `"read"` con `[i]`: informa del nivel actual del medidor `i`.

Devuelve las respuestas de las operaciones `"read"`, en orden.

### Ejemplo 1

```
readings = [10, 20, 30, 40], ops = ["add", "read", "add", "read", "read"], args = [[1, 3, 5], [2], [0, 1, -2], [1], [0]]
Salida: [35, 23, 8]
```

El medidor 2 pasa a 30 + 5 = 35. El medidor 1 recibe +5 y luego −2: 23.

### Ejemplo 2

```
readings = [0, 0], ops = ["read", "add", "read"], args = [[1], [0, 1, 7], [1]]
Salida: [0, 7]
```

### Ejemplo 3

```
readings = [5], ops = ["add", "add", "read"], args = [[0, 0, 3], [0, 0, -9], [0]]
Salida: [-1]
```

### Restricciones

- 1 ≤ readings.length ≤ 10⁵
- -10⁴ ≤ readings[i] ≤ 10⁴
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"add"` lleva `[a, b, delta]` con 0 ≤ a ≤ b < readings.length y -100 ≤ delta ≤ 100; `"read"` lleva `[i]` con 0 ≤ i < readings.length.
