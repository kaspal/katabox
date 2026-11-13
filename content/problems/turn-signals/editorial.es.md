## Enfoque: el signo de un producto cruzado

Para los vectores `u = B − A` y `v = C − B`, el producto cruzado

```
cross = u.x · v.y − u.y · v.x
```

es el área con signo del paralelogramo que forman. Es positivo cuando pasar de `u` a `v` gira en sentido antihorario (un giro a la izquierda, con y hacia el norte), negativo en sentido horario, y cero cuando `u` y `v` son paralelos, de modo que A, B y C están en una recta.

Las coordenadas son enteros de hasta 10⁴, así que todos los productos caben sin problema y la comprobación es exacta: sin ángulos y sin coma flotante.

### Paso a paso

`[0, 0, 2, 0, 2, 3]`: u = (2, 0), v = (0, 3), cross = 2·3 − 0·0 = 6 > 0, así que "left".

**Complejidad:** O(1) por ruta, O(n) en total.
