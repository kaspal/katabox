Esta vez el array puede tener **muchos** pares que suman `target`, o ninguno. Devuelve
**cuántos** pares de posiciones `i < j` cumplen `nums[i] + nums[j] == target`.

Valores iguales en posiciones distintas son pares distintos: en `[2, 2, 2]` con
`target = 4` hay tres pares.

### Ejemplo 1

```
nums = [3, 1, 2, 2, 3], target = 4
Salida: 3
```

Los pares son las posiciones (0, 1), (1, 4) y (2, 3).

### Ejemplo 2

```
nums = [-3, 0, -3, 3, 3], target = 3
Salida: 2
```

### Ejemplo 3

```
nums = [-3, -3, 1, -2], target = 1
Salida: 0
```

### Restricciones

- 1 ≤ nums.length ≤ 10⁵
- −10⁹ ≤ nums[i] ≤ 10⁹
- −2·10⁹ ≤ target ≤ 2·10⁹
- La respuesta cabe en un entero de 64 bits (y en un número de JavaScript).
