Un guardabosques quiere rodear todos los árboles de una arboleda con la cerca más corta posible: una goma elástica tensada alrededor de todos los árboles. La región cercada es la **envolvente convexa** de los árboles.

Dadas las posiciones de los árboles `trees[i] = [x, y]`, devuelve el **doble** del área que encierra esa cerca (el doble del área siempre es un número entero). Si todos los árboles están en una recta, o solo hay uno, la cerca no encierra nada: devuelve `0`.

Puede haber varios árboles en la misma posición.

### Ejemplo 1

```
trees = [[0, 0], [4, 0], [4, 3], [0, 3], [2, 1], [1, 2]]
Salida: 24
```

La cerca es el rectángulo de 4 × 3: área 12, el doble del área 24.

### Ejemplo 2

```
trees = [[1, 1], [2, 2], [3, 3]]
Salida: 0
```

### Ejemplo 3

```
trees = [[0, 0], [2, 0], [1, 1], [1, 3], [0, 0]]
Salida: 6
```

La cerca es el triángulo (0, 0), (2, 0), (1, 3), de área 3.

### Restricciones

- 1 ≤ trees.length ≤ 10⁵
- Todas las coordenadas son enteros con valor absoluto de como mucho 10⁷.
