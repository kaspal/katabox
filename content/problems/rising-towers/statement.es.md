Una fotógrafa recorre una calle de izquierda a derecha y pasa junto a torres de alturas `heights[0], heights[1], …`. Para un cartel quiere elegir algunas torres, respetando su orden en la calle, de modo que cada torre elegida sea **estrictamente más alta** que la elegida antes.

Devuelve el mayor número de torres que puede elegir.

### Ejemplo 1

```
heights = [5, 1, 8, 3, 6, 9, 4]
Output: 4
```

Por ejemplo 1, 3, 6, 9.

### Ejemplo 2

```
heights = [12, 12, 12]
Output: 1
```

Las alturas iguales no crecen.

### Ejemplo 3

```
heights = [9, 8, 2]
Output: 1
```

### Restricciones

- 1 ≤ heights.length ≤ 5·10⁴
- 1 ≤ heights[i] ≤ 10⁶
