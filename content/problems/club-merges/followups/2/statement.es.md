Esta vez los clubes pueden echarse atrás. Hay `n` jugadores, numerados del `0` al `n − 1`, y cada uno empieza siendo un club por sí solo. Aplica `ops` en orden:

- `[1, a, b]`: una fusión, como antes: los clubes de `a` y `b` se convierten en uno (no cambia nada si ya lo son).
- `[0]`: deshace la última fusión que sigue vigente y deja los clubes exactamente como estaban antes de ella. Una fusión que no cambió nada también cuenta como fusión: deshacerla tampoco cambia nada. Si no queda ninguna fusión por deshacer, `[0]` no hace nada.

Una fusión deshecha desaparece para siempre, así que el siguiente `[0]` deshace la fusión anterior a ella. Después de cada operación, devuelve el número de clubes.

### Ejemplo 1

```
n = 4, ops = [[1, 0, 1], [1, 1, 0], [1, 2, 3], [0], [0], [0], [0]]
Salida: [3, 3, 2, 3, 3, 4, 4]
```

Las vueltas atrás deshacen [2, 3], después [1, 0] (que no había cambiado nada) y después [0, 1]; la última ya no encuentra nada.

### Ejemplo 2

```
n = 5, ops = [[0], [1, 0, 2], [0], [1, 3, 3], [1, 0, 3], [0]]
Salida: [5, 4, 5, 5, 4, 5]
```

### Ejemplo 3

```
n = 5, ops = [[1, 0, 0], [0], [1, 2, 2], [1, 4, 0], [1, 1, 3], [1, 4, 2]]
Salida: [5, 5, 5, 4, 3, 2]
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 1 ≤ ops.length ≤ 10⁵
- Cada operación es `[0]` (deshacer) o `[1, a, b]` (fusión) con 0 ≤ a, b < n (a puede ser igual a b)
