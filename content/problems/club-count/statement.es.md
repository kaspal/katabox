Lucía organiza la semana de bienvenida de su universidad y sigue cómo los estudiantes nuevos se agrupan en pandillas. Su función informa del número de grupos después de cada nueva amistad, pero para el jueves ya decía que había habido más uniones de las que eran posibles.

Hay `n` personas numeradas de 0 a `n - 1`, y cada una empieza en un club propio. Los elementos de `friendships` llegan en orden; cada `[a, b]` significa que `a` y `b` se hacen amigos, y sus dos clubes pasan a ser uno (el amigo de un amigo está en el mismo club). Devuelve una lista con un número por amistad: el número de clubes justo después de añadir esa amistad. Una amistad entre dos personas que ya están en el mismo club no cambia nada.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
n = 5, friendships = [[0, 1], [2, 1], [0, 3], [2, 3], [4, 0]]
Salida: [4, 3, 2, 2, 1]
```

Después de `[2, 1]`, las personas 0, 1 y 2 comparten club. `[2, 3]` une a dos personas que ya están juntas a través de 0, así que el número se queda en 2.

### Ejemplo 2

```
n = 4, friendships = [[0,1],[0,1],[3,1],[3,0],[0,1]]
Salida: [3,3,2,2,2]
```

### Ejemplo 3

```
n = 4, friendships = [[0,1],[1,2],[2,1]]
Salida: [3,2,2]
```

### Restricciones

- 2 ≤ n ≤ 10⁴
- 1 ≤ friendships.length ≤ 2·10⁴
- Cada amistad es `[a, b]` con 0 ≤ a, b < n y a ≠ b
