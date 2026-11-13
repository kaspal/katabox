Una carretera está dividida en `length` tramos, numerados desde 0. El ayuntamiento planifica obras: cada obra `[a, b, crews]` envía `crews` operarios a cada tramo de `a` a `b`, ambos incluidos. Las obras se pueden solapar.

Devuelve cuántos operarios acaban en cada tramo.

### Ejemplo 1

```
length = 5, repairs = [[0, 2, 1], [1, 4, 2], [3, 3, 5]]
Salida: [1, 3, 3, 7, 2]
```

El tramo 3 recibe 2 operarios de la segunda obra y 5 de la tercera.

### Ejemplo 2

```
length = 3, repairs = []
Salida: [0, 0, 0]
```

### Ejemplo 3

```
length = 1, repairs = [[0, 0, 4], [0, 0, 4]]
Salida: [8]
```

### Restricciones

- 1 ≤ length ≤ 10⁵
- 0 ≤ repairs.length ≤ 10⁵
- 0 ≤ a ≤ b < length y 1 ≤ crews ≤ 100 para cada obra [a, b, crews]
