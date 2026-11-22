Con el mismo tipo de árbol genealógico, la asociación quiere saber ahora lo cerca que está cada pareja de personas. La **distancia** entre dos personas es el número de enlaces de parentesco que hay en el camino de una a otra por el árbol: sube desde cada una hasta el primer antepasado que comparten (que puede ser una de las dos) y suma los dos números de pasos.

Para cada `pairs[i] = [a, b]`, devuelve la distancia entre `a` y `b`, en orden. Una persona está a distancia 0 de sí misma, y una madre o un padre y su hijo están a distancia 1.

### Ejemplo 1

```
parent = [-1, 0, 0, 1, 1, 2], pairs = [[3, 4], [3, 5], [0, 0], [5, 2]]
Salida: [2, 4, 0, 1]
```

Las personas 3 y 4 comparten a la persona 1 como progenitora. Las personas 3 y 5 solo se encuentran en la fundadora, a 2 pasos hacia arriba desde cada una.

### Ejemplo 2

```
parent = [3, 3, -1, 2, 0, 3], pairs = [[0, 3], [0, 3], [3, 4]]
Salida: [1, 1, 2]
```

La misma pareja puede preguntarse más de una vez.

### Ejemplo 3

```
parent = [5, 3, -1, 2, 1, 3], pairs = [[0, 4], [5, 1], [3, 5]]
Salida: [4, 2, 1]
```

### Restricciones

- 1 ≤ n ≤ 10⁵, donde n = parent.length
- Exactamente un `parent[i]` es `-1`; cada uno de los demás está entre `0` y `n − 1`
- Siguiendo `parent` desde cualquiera siempre se llega a la fundadora (no hay bucles)
- 1 ≤ pairs.length ≤ 10⁵
- `pairs[i] = [a, b]` con 0 ≤ a, b < n (a puede ser igual a b)
