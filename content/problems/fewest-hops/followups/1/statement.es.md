La red de radio es la misma: `n` estaciones numeradas de `0` a `n − 1`, y cada par
`[a, b]` de `links` permite que las estaciones `a` y `b` hablen directamente, en los dos
sentidos, con un **salto**. Esta vez ningún enlace aparece dos veces.

El equipo de la red ya sabe cuál es el menor número de saltos de la estación `0` a la
estación `n − 1`. Ahora quiere saber lo robusto que es: **¿cuántas rutas distintas** llegan
a la estación `n − 1` desde la `0` con exactamente ese menor número de saltos? Dos rutas
son distintas cuando sus secuencias de estaciones son distintas.

La cuenta puede ser enorme: devuélvela módulo 1 000 003. Devuelve `0` si no se puede llegar
a la estación `n − 1`. Cuando `n = 1`, la ruta vacía (cero saltos) es la única ruta.

### Ejemplo 1

```
n = 6, links = [[0, 1], [0, 2], [1, 3], [2, 3], [1, 4], [2, 4], [3, 5], [4, 5]]
Salida: 4
```

Todas las rutas más cortas dan 3 saltos: 0 → 1 o 2, luego → 3 o 4, luego → 5. Son
2 · 2 = 4 rutas.

### Ejemplo 2

```
n = 5, links = [[0, 1], [1, 4], [0, 2], [2, 3], [3, 4]]
Salida: 1
```

0 → 1 → 4 da 2 saltos; la ruta por 2 y 3 da 3, así que no cuenta.

### Ejemplo 3

```
n = 3, links = [[0, 1]]
Salida: 0
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` con 0 ≤ a, b < n y a ≠ b
- Ningún par de estaciones está enlazado dos veces (en ningún orden).
