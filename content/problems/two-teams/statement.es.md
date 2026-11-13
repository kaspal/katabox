Un torneo de fútbol sala tiene `n` jugadores, numerados de 0 a n − 1, que hay que repartir en dos equipos. Algunas parejas de jugadores son rivales, `rivalries[i] = [a, b]`, y los rivales deben jugar en equipos **distintos**.

Devuelve si existe ese reparto. Los jugadores sin rivales pueden ir a cualquier equipo, y un equipo puede quedar vacío.

### Ejemplo 1

```
n = 4, rivalries = [[0, 1], [1, 2], [2, 3], [3, 0]]
Salida: true
```

Los jugadores 0 y 2 contra 1 y 3.

### Ejemplo 2

```
n = 3, rivalries = [[0, 1], [1, 2], [2, 0]]
Salida: false
```

Tres jugadores rivales entre sí no caben en dos equipos.

### Ejemplo 3

```
n = 5, rivalries = []
Salida: true
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ rivalries.length ≤ 2·10⁵
- 0 ≤ a, b < n, a ≠ b, y ninguna pareja aparece dos veces.
