Los mismos pueblos y carreteras de antes, pero ahora el mensajero tiene **un pase** que
deja gratis una sola carretera a su elección: en esa carretera el peaje es 0. También
puede no usar el pase. Devuelve el peaje total más barato para ir del pueblo `start` al
pueblo `goal`, o `-1` si no se puede llegar a `goal`.

Cada entrada `[a, b, toll]` de `roads` es una carretera entre los pueblos `a` y `b` que se
puede recorrer en los dos sentidos pagando un peaje `toll`; puede haber varias carreteras
entre los mismos dos pueblos. Ir de un pueblo a sí mismo cuesta 0.

### Ejemplo 1

```
n = 4, roads = [[0, 1, 5], [1, 2, 7], [2, 3, 2], [0, 2, 20]], start = 0, goal = 3
Salida: 2
```

Usa el pase en la carretera cara 0 → 2 y paga 2 por 2 → 3. La ruta 0 → 1 → 2 → 3 cuesta
14, y aún 7 con el pase en 1 → 2.

### Ejemplo 2

```
n = 5, roads = [[1, 0, 5], [0, 3, 8], [3, 4, 4], [0, 3, 1], [3, 4, 1], [3, 2, 4]], start = 4, goal = 0
Salida: 1
```

### Ejemplo 3

```
n = 5, roads = [[0, 4, 2], [2, 1, 5], [2, 1, 1], [4, 1, 7], [3, 2, 9], [3, 2, 1]], start = 0, goal = 2
Salida: 3
```

0 → 4 → 1 → 2 cuesta 2 + 7 + 1 = 10; con el pase en 4 → 1 quedan 2 + 1 = 3.

### Restricciones

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ roads.length ≤ 5·10⁴
- roads[i] = [a, b, toll] con 0 ≤ a, b < n, a ≠ b y 1 ≤ toll ≤ 10⁴
- 0 ≤ start, goal < n
