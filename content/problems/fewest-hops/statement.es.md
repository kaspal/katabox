Un mensaje tiene que cruzar una red de `n` estaciones de radio, numeradas del `0` al `n − 1`. Cada pareja `[a, b]` de `links` significa que las estaciones `a` y `b` pueden hablar directamente (en ambos sentidos). Pasar el mensaje por un enlace es un **salto**.

¿Cuál es el menor número de saltos para llevar un mensaje de la estación `0` a la estación `n − 1`? Devuelve `-1` si no puede llegar de ninguna forma.

### Ejemplo 1

```
n = 5, links = [[0, 1], [1, 2], [2, 4], [0, 3], [3, 4]]
Salida: 2
```

0 → 3 → 4 cuesta dos saltos; la ruta por 1 y 2 cuesta tres.

### Ejemplo 2

```
n = 4, links = [[0, 1], [2, 3]]
Salida: -1
```

### Ejemplo 3

```
n = 1, links = []
Salida: 0
```

El mensaje empieza donde tiene que llegar.

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` con 0 ≤ a, b < n y a ≠ b; un enlace puede aparecer dos veces
