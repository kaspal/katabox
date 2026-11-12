Un valle tiene `n` pueblos, numerados del `0` al `n − 1`, unidos por exactamente `n` carreteras de doble sentido; `roads[i] = [a, b]` une los pueblos `a` y `b`. Desde cada pueblo se puede llegar a todos los demás. Ninguna carretera empieza y termina en el mismo pueblo, y no hay dos carreteras que unan la misma pareja.

Una red conexa con tantas carreteras como pueblos contiene **exactamente un bucle**: un recorrido que pasa una vez por algunos pueblos y vuelve a su inicio sin repetir carretera. El ayuntamiento quiere convertir ese bucle en una carretera de circunvalación. ¿Cuántos pueblos están **en el bucle**?

### Ejemplo 1

```
n = 5, roads = [[0, 1], [1, 2], [2, 0], [2, 3], [3, 4]]
Salida: 3
```

El bucle es 0 → 1 → 2 → 0. Los pueblos 3 y 4 cuelgan del pueblo 2.

### Ejemplo 2

```
n = 4, roads = [[0, 1], [1, 2], [2, 3], [3, 0]]
Salida: 4
```

### Ejemplo 3

```
n = 7, roads = [[0, 1], [1, 2], [2, 3], [3, 0], [2, 4], [4, 5], [0, 6]]
Salida: 4
```

### Restricciones

- 3 ≤ n ≤ 10⁵, roads.length = n
- `roads[i] = [a, b]` con 0 ≤ a, b < n y a ≠ b; ninguna pareja de pueblos está unida dos veces
- Desde cada pueblo se puede llegar a todos los demás
