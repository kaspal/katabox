Esta vez los registros del refugio se repiten cada año: después del último día de `temps` vuelve a venir el primero. Para cada día, cuenta cuántos días avanzas, dando la vuelta del final al principio, hasta el primer mediodía **estrictamente más cálido**.

Devuelve un arreglo `wait` de la misma longitud, donde `wait[i]` es ese número de días (como mucho `temps.length − 1`), o `0` si ningún otro día de la temporada es más cálido que el día `i`.

### Ejemplo 1

```
temps = [4, 2, 9, 1, 3]
Salida: [2, 1, 0, 1, 1]
```

El día 4 (3°) da la vuelta hasta el día 0 (4°), un día después; el día 2 (9°) nunca se supera.

### Ejemplo 2

```
temps = [-5, 9, -7, -1, -6, 6]
Salida: [1, 0, 1, 2, 1, 2]
```

### Ejemplo 3

```
temps = [-8, -7, -7, 2, -4, 0]
Salida: [1, 2, 1, 0, 1, 4]
```

El día 5 (0°) da la vuelta, pasa los días 0, 1 y 2 y llega al día 3 (2°): 4 días. El día 1 espera 2 días, porque el día 2 es igual de frío, no más cálido.

### Restricciones

- 1 ≤ temps.length ≤ 10⁵
- −50 ≤ temps[i] ≤ 50
