Una tienda lleva un libro con cuántos artículos hay en cada estante: `stock[i]` para el estante `i` (desde 0). El día trae una lista de operaciones, `ops[j]` con sus números `args[j]`:

- `"set"` con `[i, count]`: el estante `i` pasa a tener `count` artículos.
- `"total"` con `[a, b]`: informa de cuántos artículos hay en los estantes de `a` a `b`, ambos incluidos.

Devuelve las respuestas de las operaciones `"total"`, en orden.

### Ejemplo 1

```
stock = [5, 3, 8, 2], ops = ["total", "set", "total", "total"], args = [[0, 3], [2, 1], [1, 2], [2, 2]]
Salida: [18, 4, 1]
```

Todos los estantes suman 18 artículos. Cuando el estante 2 baja a 1, los estantes 1 y 2 suman 3 + 1 = 4.

### Ejemplo 2

```
stock = [0, 0, 0], ops = ["set", "set", "total"], args = [[0, 7], [2, 4], [0, 2]]
Salida: [11]
```

### Ejemplo 3

```
stock = [6], ops = ["total"], args = [[0, 0]]
Salida: [6]
```

### Restricciones

- 1 ≤ stock.length ≤ 10⁵
- 0 ≤ stock[i], count ≤ 10⁴
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"set"` lleva `[i, count]` con 0 ≤ i < stock.length; `"total"` lleva `[a, b]` con 0 ≤ a ≤ b < stock.length.
