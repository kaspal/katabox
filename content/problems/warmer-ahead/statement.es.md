Un refugio de montaña anota la temperatura del mediodía de cada día de la temporada en `temps`. Los escaladores quieren saber, para cada día, cuánto tendrían que esperar hasta un mediodía **estrictamente más cálido**.

Devuelve un arreglo `wait` de la misma longitud, donde `wait[i]` es el número de días desde el día `i` hasta el primer día posterior con una temperatura mayor, o `0` si ningún día posterior es más cálido.

### Ejemplo 1

```
temps = [3, 1, 4, 1, 5, 2]
Salida: [2, 1, 2, 1, 0, 0]
```

El día 0 (3°) espera hasta el día 2 (4°); el día 4 (5°) nunca se supera.

### Ejemplo 2

```
temps = [-5, -5, -2]
Salida: [2, 1, 0]
```

Una temperatura igual no es más cálida, así que el día 0 espera dos días.

### Ejemplo 3

```
temps = [9, 8, 7]
Salida: [0, 0, 0]
```

### Restricciones

- 1 ≤ temps.length ≤ 10⁵
- −50 ≤ temps[i] ≤ 50
