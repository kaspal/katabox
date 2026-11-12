El termostato de un invernadero registra una lectura de temperatura por minuto en `readings`. Para detectar periodos inestables, la jardinera mira cada tramo de `k` lecturas consecutivas y mide su **oscilación**: la lectura más alta del tramo menos la más baja.

Devuelve la oscilación de cada tramo de `k` lecturas, desde el primer tramo (lecturas de 0 a k − 1) hasta el último.

### Ejemplo 1

```
readings = [3, 1, 4, 1, 5, 9, 2, 6], k = 3
Salida: [3, 3, 4, 8, 7, 7]
```

El primer tramo [3, 1, 4] oscila 4 − 1 = 3; el tramo [1, 5, 9] oscila 8.

### Ejemplo 2

```
readings = [5], k = 1
Salida: [0]
```

### Ejemplo 3

```
readings = [2, 2, 2, 2], k = 2
Salida: [0, 0, 0]
```

### Restricciones

- 1 ≤ k ≤ readings.length ≤ 10⁵
- −10⁴ ≤ readings[i] ≤ 10⁴
