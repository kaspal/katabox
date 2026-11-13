Un archipiélago tiene `n` puertos, numerados de 0 a n − 1. Cada ruta de ferri `[a, b, minutes]` navega entre los puertos `a` y `b` en cualquier sentido y tarda `minutes`. Puede haber varias rutas entre los mismos dos puertos.

Para cada consulta `[from, to]`, devuelve el menor tiempo total de navegación de `from` a `to` (cambiar de ferri es instantáneo), `0` cuando `from = to`, o `-1` si no se puede llegar a `to`.

### Ejemplo 1

```
n = 4, routes = [[0, 1, 30], [1, 2, 20], [0, 2, 70], [2, 3, 10]], queries = [[0, 2], [3, 0], [1, 1]]
Salida: [50, 60, 0]
```

De 0 a 2, pasar por 1 (30 + 20) es mejor que la ruta directa (70).

### Ejemplo 2

```
n = 3, routes = [[0, 1, 5]], queries = [[0, 2], [1, 0]]
Salida: [-1, 5]
```

### Ejemplo 3

```
n = 2, routes = [[0, 1, 9], [1, 0, 4]], queries = [[0, 1]]
Salida: [4]
```

### Restricciones

- 1 ≤ n ≤ 100
- 0 ≤ routes.length ≤ 5000, con 0 ≤ a, b < n, a ≠ b y 1 ≤ minutes ≤ 10⁴
- 1 ≤ queries.length ≤ 10⁴, con 0 ≤ from, to < n
