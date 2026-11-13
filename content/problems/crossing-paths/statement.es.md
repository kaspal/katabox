Un urbanista dibuja los senderos de un parque como segmentos rectos y quiere saber qué parejas de senderos se encuentran, para pavimentar el cruce. Cada entrada de `pairs` es `[x1, y1, x2, y2, x3, y3, x4, y4]`: el primer sendero va de (x1, y1) a (x2, y2) y el segundo de (x3, y3) a (x4, y4).

Dos senderos **se encuentran** cuando comparten al menos un punto: se cruzan, uno termina sobre el otro, comparten un extremo o se solapan a lo largo de una recta. Devuelve `true` o `false` para cada pareja.

### Ejemplo 1

```
pairs = [[0, 0, 4, 4, 0, 4, 4, 0], [0, 0, 2, 2, 3, 3, 5, 5], [0, 0, 2, 0, 2, 0, 2, 5]]
Salida: [true, false, true]
```

Las diagonales de un cuadrado se cruzan. La segunda pareja está en una misma recta pero deja un hueco. La tercera comparte el punto (2, 0).

### Ejemplo 2

```
pairs = [[0, 0, 4, 0, 2, 0, 6, 0], [0, 0, 1, 1, 1, 0, 2, -1]]
Salida: [true, false]
```

La primera pareja se solapa de x = 2 a x = 4.

### Ejemplo 3

```
pairs = [[-2, 3, 2, 3, 0, 5, 0, 4]]
Salida: [false]
```

### Restricciones

- 1 ≤ pairs.length ≤ 10⁴
- Todas las coordenadas son enteros con valor absoluto de como mucho 10⁴.
- Cada sendero tiene dos extremos distintos.
