La red de radio es la misma: `n` estaciones numeradas del `0` al `n − 1`, y cada pareja
`[a, b]` de `links` permite que las estaciones `a` y `b` hablen directamente en ambos
sentidos, un **salto** por enlace. Esta vez el mensaje sale de la estación `0` y tiene que
llegar a **todas** las estaciones.

Devuelve un array `hops` de longitud `n` donde `hops[i]` es el menor número de saltos de
la estación `0` a la estación `i`, o `-1` si el mensaje no puede llegar a la estación `i`
de ninguna forma. `hops[0]` es `0`.

### Ejemplo 1

```
n = 6, links = [[0, 1], [1, 2], [0, 3], [3, 2], [4, 5]]
Salida: [0, 1, 2, 1, -1, -1]
```

Las estaciones 4 y 5 solo hablan entre sí, así que el mensaje nunca llega a ellas.

### Ejemplo 2

```
n = 4, links = [[2, 0], [3, 1], [3, 2]]
Salida: [0, 3, 1, 2]
```

### Ejemplo 3

```
n = 4, links = [[0, 1], [1, 2], [2, 1]]
Salida: [0, 1, 2, -1]
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` con 0 ≤ a, b < n y a ≠ b; un enlace puede aparecer dos veces
