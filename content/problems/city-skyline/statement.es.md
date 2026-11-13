Una artista de postales dibuja el perfil de una ciudad desde lejos. El edificio `i` es un rectángulo apoyado en el suelo desde `x = left` hasta `x = right` con altura `height`: `buildings[i] = [left, right, height]`. El perfil en una posición `x` es la altura del edificio más alto que la cubre (`left ≤ x < right`), o 0 si no hay ninguno.

Devuelve el perfil como sus **puntos clave** `[x, h]`: cada posición `x` en la que cambia la altura del perfil, con `h` la nueva altura a partir de `x`. Enuméralos por `x` creciente; dos puntos clave consecutivos nunca tienen la misma altura.

### Ejemplo 1

```
buildings = [[1, 4, 3], [2, 6, 5], [8, 9, 2]]
Salida: [[1, 3], [2, 5], [6, 0], [8, 2], [9, 0]]
```

### Ejemplo 2

```
buildings = [[0, 2, 4], [2, 5, 4]]
Salida: [[0, 4], [5, 0]]
```

Los dos edificios se juntan en x = 2 con la misma altura, así que el perfil no cambia ahí.

### Ejemplo 3

```
buildings = [[3, 7, 6], [4, 5, 2]]
Salida: [[3, 6], [7, 0]]
```

### Restricciones

- 1 ≤ buildings.length ≤ 10⁵
- 0 ≤ left < right ≤ 10⁶
- 1 ≤ height ≤ 10⁴
