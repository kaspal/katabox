Una empresa de mensajería cubre `n` pueblos, numerados de `0` a `n − 1`. Cada entrada `[a, b, toll]` de `roads` es una carretera entre los pueblos `a` y `b` que se puede recorrer en los dos sentidos pagando un peaje `toll`. Puede haber varias carreteras entre los mismos dos pueblos.

Devuelve el peaje total más barato para ir del pueblo `start` al pueblo `goal`, o `-1` si no se puede llegar a `goal`. Ir de un pueblo a sí mismo cuesta 0.

### Ejemplo 1

```
n = 4, roads = [[0, 1, 5], [1, 3, 5], [0, 2, 2], [2, 3, 6], [1, 2, 1]], start = 0, goal = 3
Salida: 8
```

0 → 2 → 3 cuesta 2 + 6 = 8, más barato que 0 → 1 → 3 (10). La ruta 0 → 2 → 1 → 3 también cuesta 2 + 1 + 5 = 8.

### Ejemplo 2

```
n = 3, roads = [[0, 1, 4]], start = 0, goal = 2
Salida: -1
```

### Ejemplo 3

```
n = 2, roads = [[1, 0, 7], [0, 1, 3]], start = 1, goal = 1
Salida: 0
```

### Restricciones

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ roads.length ≤ 5·10⁴
- roads[i] = [a, b, toll] con 0 ≤ a, b < n, a ≠ b y 1 ≤ toll ≤ 10⁴
- 0 ≤ start, goal < n
