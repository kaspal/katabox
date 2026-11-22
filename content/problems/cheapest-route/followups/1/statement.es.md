Los mismos pueblos y carreteras de antes, pero ahora el mensajero puede recorrer **como
mucho `k` carreteras** en el camino. Devuelve el peaje total más barato para ir del pueblo
`start` al pueblo `goal` usando como mucho `k` carreteras, o `-1` si no se puede llegar a
`goal` con esa cantidad de carreteras.

Cada entrada `[a, b, toll]` de `roads` es una carretera entre los pueblos `a` y `b` que se
puede recorrer en los dos sentidos pagando un peaje `toll`; puede haber varias carreteras
entre los mismos dos pueblos. Una carretera recorrida dos veces cuenta (y se paga) dos
veces. Si `start == goal`, la respuesta es 0.

### Ejemplo 1

```
n = 4, roads = [[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 10]], start = 0, goal = 3, k = 2
Salida: 10
```

0 → 1 → 2 → 3 cuesta solo 3, pero usa tres carreteras; con dos como mucho, la carretera
directa (10) es la única opción.

### Ejemplo 2

```
n = 5, roads = [[1, 0, 5], [0, 3, 8], [3, 4, 4], [0, 3, 1], [3, 4, 1], [3, 2, 4]], start = 4, goal = 0, k = 2
Salida: 2
```

### Ejemplo 3

```
n = 5, roads = [[0, 4, 2], [2, 1, 5], [2, 1, 1], [4, 1, 7], [3, 2, 9], [3, 2, 1]], start = 0, goal = 2, k = 2
Salida: -1
```

La única ruta es 0 → 4 → 1 → 2, que necesita tres carreteras.

### Restricciones

- 1 ≤ n ≤ 1000
- 0 ≤ roads.length ≤ 10⁴
- roads[i] = [a, b, toll] con 0 ≤ a, b < n, a ≠ b y 1 ≤ toll ≤ 10⁴
- 0 ≤ start, goal < n
- 1 ≤ k ≤ 50
