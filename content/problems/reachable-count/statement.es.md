Kenji diseña una sala de escape con `n` salas numeradas de 0 a n - 1. Escribió una función que le dice a cuántas salas puede llegar un jugador desde la sala inicial, pero siempre le promete más salas de las que tiene su plano.

Cada puerta `[a, b]` de `doors` une las salas `a` y `b` y se puede cruzar en ambos sentidos; el mismo par de salas puede aparecer más de una vez. Devuelve cuántas salas se pueden alcanzar desde la sala `start`, contando la propia `start`.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
n = 5, doors = [[0, 1], [1, 2], [3, 4]], start = 1
Salida: 3
```

Desde la sala 1 el jugador llega a las salas 0, 1 y 2.

### Ejemplo 2

```
n = 4, doors = [[0,1],[0,1],[3,1],[3,0]], start = 0
Salida: 3
```

La sala 2 no tiene puertas.

### Ejemplo 3

```
n = 4, doors = [[0,1],[1,2]], start = 2
Salida: 3
```

### Restricciones

- 1 ≤ n ≤ 10⁴
- 0 ≤ doors.length ≤ 2·10⁴
- doors[i] = [a, b] con 0 ≤ a, b < n y a ≠ b
- 0 ≤ start < n
