Una app de autoescuela muestra a cada alumno una ruta corta en un mapa: desde el punto A conduce en línea recta hasta el punto B, y después en línea recta hasta el punto C. En B, el alumno debe poner el intermitente **izquierdo**, el **derecho**, o ninguno.

Cada ruta es `[ax, ay, bx, by, cx, cy]`, con el eje x hacia el este y el eje y hacia el norte. Devuelve, para cada ruta, una de estas cadenas exactas:

- `"left"` si el giro en B es a la izquierda (en sentido antihorario),
- `"right"` si es a la derecha (en sentido horario),
- `"straight"` si A, B y C están en una misma recta (también si se da la vuelta sobre la misma recta).

### Ejemplo 1

```
routes = [[0, 0, 2, 0, 2, 3], [0, 0, 0, 2, 3, 2], [1, 1, 2, 2, 5, 5]]
Salida: ["left", "right", "straight"]
```

La primera ruta va hacia el este y después hacia el norte: un giro a la izquierda.

### Ejemplo 2

```
routes = [[0, 0, 4, 0, 1, 0]]
Salida: ["straight"]
```

C está detrás de B, en la misma recta.

### Ejemplo 3

```
routes = [[-3, 5, 1, 1, 0, -4]]
Salida: ["right"]
```

### Restricciones

- 1 ≤ routes.length ≤ 10⁴
- Cada ruta tiene 6 enteros con valor absoluto de como mucho 10⁴.
- A ≠ B y B ≠ C.
