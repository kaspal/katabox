Esta vez los estantes no se fijan de uno en uno: una **entrega** añade el mismo número de
artículos a cada estante de una fila de estantes consecutivos. `stock[i]` es la cantidad
inicial del estante `i` (desde 0), y el día trae operaciones `ops[j]` con sus números
`args[j]`:

- `"add"` con `[a, b, count]`: cada uno de los estantes de `a` a `b`, ambos incluidos,
  recibe `count` artículos más.
- `"total"` con `[a, b]`: informa de cuántos artículos hay en los estantes de `a` a `b`,
  ambos incluidos.

Devuelve las respuestas de las operaciones `"total"`, en orden.

### Ejemplo 1

```
stock = [4, 0, 7, 1], ops = ["total", "add", "total", "total"], args = [[0, 3], [1, 2, 5], [0, 3], [2, 2]]
Salida: [12, 22, 12]
```

La entrega pone 5 artículos más en los estantes 1 y 2, así que los estantes tienen 4, 5,
12 y 1.

### Ejemplo 2

```
stock = [4, 18, 2, 8, 3], ops = ["add", "add", "total", "add", "total"], args = [[3, 3, 6], [0, 1, 6], [3, 4], [0, 3, 3], [0, 4]]
Salida: [17, 65]
```

### Ejemplo 3

```
stock = [1, 2, 2, 11, 5], ops = ["total", "total", "total", "total", "add"], args = [[2, 2], [0, 4], [1, 3], [2, 4], [3, 4, 0]]
Salida: [2, 21, 15, 18]
```

### Restricciones

- 1 ≤ stock.length ≤ 10⁵
- 0 ≤ stock[i], count ≤ 10⁴
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"add"` lleva `[a, b, count]` y `"total"` lleva `[a, b]`, con 0 ≤ a ≤ b < stock.length.
- Un total cabe en un entero de 64 bits (y en un número de JavaScript).
