Tomás anota cada día cuántos pasos ganó o perdió, y escribió una función rápida para sumar los días de `a` a `b`. Casi siempre los totales cuadran, pero de vez en cuando salen algo desviados y no entiende por qué.

Dado un arreglo de enteros `nums` y dos índices `a` y `b`, devuelve la suma de `nums[a]`, `nums[a + 1]`, ..., `nums[b]`. Los dos extremos cuentan.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
nums = [4, -2, 7, 3, 5], a = 1, b = 3
Salida: 8
```

-2 + 7 + 3 = 8.

### Ejemplo 2

```
nums = [16, -16, -4, -13, 11], a = 3, b = 4
Salida: -2
```

### Ejemplo 3

```
nums = [-15, -15, 3, -10, -1], a = 2, b = 4
Salida: -8
```

### Restricciones

- 1 ≤ nums.length ≤ 10⁵
- −1000 ≤ nums[i] ≤ 1000
- 0 ≤ a ≤ b < nums.length
