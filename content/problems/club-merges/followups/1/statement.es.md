Las mismas fusiones de antes, pero ahora el pueblo quiere saber el tamaño de su **club más grande** en lugar del número de clubes. Hay `n` jugadores, numerados del `0` al `n − 1`, y cada uno empieza siendo un club por sí solo; `merges[i] = [a, b]` une los clubes de los jugadores `a` y `b` (no cambia nada si ya están juntos).

Después de cada fusión, devuelve el número de jugadores del club más grande, en orden.

### Ejemplo 1

```
n = 5, merges = [[0, 1], [3, 4], [1, 0], [1, 4]]
Salida: [2, 2, 2, 4]
```

La última fusión une {0, 1} con {3, 4}.

### Ejemplo 2

```
n = 5, merges = [[1, 4], [0, 2], [0, 3], [3, 3]]
Salida: [2, 2, 3, 3]
```

### Ejemplo 3

```
n = 5, merges = [[0, 0], [0, 2], [1, 2], [2, 4]]
Salida: [1, 2, 3, 4]
```

Fusionar a un jugador consigo mismo no cambia nada, así que el club más grande sigue teniendo 1 jugador.

### Restricciones

- 1 ≤ n ≤ 10⁵
- 1 ≤ merges.length ≤ 10⁵
- `merges[i] = [a, b]` con 0 ≤ a, b < n (a puede ser igual a b)
