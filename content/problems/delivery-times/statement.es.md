Kofi planifica las rutas de un servicio de mensajería en bicicleta y escribió una función que dice a los ciclistas cuánto se tarda desde el almacén hasta cada punto de entrega. Pronto los ciclistas notaron que algunos días llegaban a un punto antes de lo que la función decía que era posible.

Hay `n` puntos numerados de 0 a `n - 1`. Cada elemento de `roads` es `[a, b, minutes]`: una calle de doble sentido entre los puntos `a` y `b` que se recorre en `minutes` minutos en cualquier dirección. Dos puntos pueden estar unidos por más de una calle. Devuelve una lista de `n` números donde el elemento `i` es la menor cantidad de minutos necesaria para ir del punto `start` al punto `i`, o `-1` si no se puede llegar al punto `i`. El elemento `start` vale `0`.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
n = 4, roads = [[0, 1, 10], [0, 2, 2], [2, 1, 3]], start = 0
Salida: [0, 5, 2, -1]
```

Ir por 0 → 2 → 1 cuesta 2 + 3 = 5 minutos, menos que la calle directa de 10. Ninguna calle llega al punto 3.

### Ejemplo 2

```
n = 4, roads = [[0,1,2],[3,1,8],[3,0,2],[3,0,7],[3,2,1],[3,1,4]], start = 0
Salida: [0,2,3,2]
```

### Ejemplo 3

```
n = 4, roads = [[0,1,3],[2,1,4],[0,2,3],[3,2,7]], start = 2
Salida: [3,4,0,7]
```

### Restricciones

- 1 ≤ n ≤ 10⁴
- 0 ≤ roads.length ≤ 3·10⁴
- Cada calle es `[a, b, minutes]` con 0 ≤ a, b < n, a ≠ b y 1 ≤ minutes ≤ 1000
- 0 ≤ start < n
