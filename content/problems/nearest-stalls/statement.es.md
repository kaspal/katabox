En un mercado de comida, la entrada está en el origen (0, 0) y el puesto `i` está en `stalls[i] = [x, y]`. Un visitante con poco tiempo quiere los `k` puestos más cercanos a la entrada, en línea recta.

Devuelve los índices de los `k` puestos más cercanos, del más cercano al más lejano. Cuando dos puestos están igual de lejos, va primero el de menor índice (y es el que gana un sitio entre los `k`).

### Ejemplo 1

```
stalls = [[3, 4], [1, 1], [-2, 0], [0, -1], [5, 5]], k = 3
Salida: [3, 1, 2]
```

Las distancias al cuadrado son 25, 2, 4, 1 y 50.

### Ejemplo 2

```
stalls = [[1, 0], [0, 1], [-1, 0]], k = 2
Salida: [0, 1]
```

Los tres puestos están a distancia 1; ganan los índices menores.

### Ejemplo 3

```
stalls = [[7, -7]], k = 1
Salida: [0]
```

### Restricciones

- 1 ≤ k ≤ stalls.length ≤ 10⁵
- Todas las coordenadas son enteros con valor absoluto de como mucho 10⁴.
