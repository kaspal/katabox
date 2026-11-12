Un pueblo tiene `n` jugadores de ajedrez, numerados del `0` al `n − 1`, y al principio cada uno es un club por sí solo. Después los clubes empiezan a fusionarse: `merges[i] = [a, b]` significa que el club del jugador `a` y el club del jugador `b` se convierten en un solo club. Si los dos jugadores ya están en el mismo club, no cambia nada.

Después de cada fusión, ¿cuántos clubes hay? Devuelve las cuentas en orden.

### Ejemplo 1

```
n = 5, merges = [[0, 1], [3, 4], [1, 0], [1, 4]]
Salida: [4, 3, 3, 2]
```

Tras la tercera fusión, 1 y 0 ya están en el mismo club, así que no cambia nada. La última fusión une {0, 1} con {3, 4}, y quedan ese club y el del jugador 2.

### Ejemplo 2

```
n = 3, merges = [[2, 2]]
Salida: [3]
```

### Ejemplo 3

```
n = 4, merges = [[0, 1], [2, 3], [0, 2]]
Salida: [3, 2, 1]
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 1 ≤ merges.length ≤ 10⁵
- `merges[i] = [a, b]` con 0 ≤ a, b < n (a puede ser igual a b)
