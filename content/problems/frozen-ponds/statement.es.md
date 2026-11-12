Una guarda forestal dibuja un humedal helado como una cuadrícula de cadenas, `ice`, todas de la misma longitud: `#` es una celda helada y `.` es agua o tierra. Dos celdas heladas son del mismo **estanque** cuando se tocan por un lado (arriba, abajo, izquierda o derecha); tocarse solo por una esquina no cuenta.

Devuelve el tamaño de cada estanque, en celdas, de **mayor a menor**. Devuelve una lista vacía si no hay nada helado.

### Ejemplo 1

```
ice = ["##..#", "#...#", "..#..", ".##.."]
Salida: [3, 3, 2]
```

El estanque de arriba a la izquierda tiene 3 celdas, el de abajo tiene 3 y el del borde derecho tiene 2.

### Ejemplo 2

```
ice = ["#.#", ".#.", "#.#"]
Salida: [1, 1, 1, 1, 1]
```

Las vecinas en diagonal son estanques distintos.

### Ejemplo 3

```
ice = ["...", "..."]
Salida: []
```

### Restricciones

- 1 ≤ ice.length ≤ 400
- 1 ≤ ice[i].length ≤ 400, todas las filas de la misma longitud
- Cada carácter es `#` o `.`
