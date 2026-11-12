Para la foto de clase, los niños se ponen en una fila y tienen que quedar ordenados del más bajo al más alto (los de la misma altura pueden quedar en cualquier orden). El fotógrafo solo permite un tipo de movimiento: dos niños que están **uno al lado del otro** se cambian de sitio.

`heights` recoge las alturas de los niños de izquierda a derecha. Devuelve el **menor número de cambios** necesario.

### Ejemplo 1

```
heights = [3, 1, 2]
Salida: 2
```

Cambia el 3 con el 1 y después el 3 con el 2: `[1, 3, 2]` y luego `[1, 2, 3]`.

### Ejemplo 2

```
heights = [1, 2, 2, 3]
Salida: 0
```

### Ejemplo 3

```
heights = [4, 3, 2, 1]
Salida: 6
```

### Restricciones

- 1 ≤ heights.length ≤ 10⁵
- 1 ≤ heights[i] ≤ 10⁹
