Una región tiene `n` pueblos, numerados de 0 a n − 1, unidos por carreteras de peaje de sentido único. La carretera `[a, b, cost]` va de `a` a `b`; su `cost` es lo que pagas, y es **negativo** cuando la región reembolsa a quien la usa.

Desde el pueblo `start`, encuentra el coste total más barato para llegar a cada pueblo:

- el coste para llegar al propio `start` es 0;
- un pueblo al que no se puede llegar recibe `1000000000`;
- si puedes llegar a un circuito de carreteras cuyo coste total es negativo (así que dar vueltas en él para siempre haría cualquier coste tan bajo como se quiera), devuelve una **lista vacía**.

Devuelve la lista de los n costes.

### Ejemplo 1

```
n = 4, roads = [[0, 1, 4], [0, 2, 5], [2, 1, -3], [1, 3, 2]], start = 0
Salida: [0, 2, 5, 4]
```

Ir 0 → 2 → 1 cuesta 5 − 3 = 2, menos que la carretera directa a 1.

### Ejemplo 2

```
n = 3, roads = [[0, 1, 1], [1, 2, -2], [2, 1, 1]], start = 0
Salida: []
```

El circuito 1 → 2 → 1 cuesta −1 y se puede alcanzar desde 0.

### Ejemplo 3

```
n = 3, roads = [[1, 2, -5]], start = 0
Salida: [0, 1000000000, 1000000000]
```

### Restricciones

- 1 ≤ n ≤ 500
- 0 ≤ roads.length ≤ 3000, con 0 ≤ a, b < n y a ≠ b
- -1000 ≤ cost ≤ 1000
- 0 ≤ start < n
