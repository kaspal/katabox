La receta más larga de un libro de cocina tiene `n` pasos, numerados del `0` al `n − 1`. Cada pareja `[a, b]` de `before` dice que el paso `a` tiene que terminar antes de que pueda empezar el paso `b`. El cocinero hace los pasos de uno en uno y, cuando hay varios listos, siempre hace el de **número más bajo**.

Devuelve los pasos en el orden en que el cocinero los hace. Si las reglas se contradicen y algunos pasos no pueden empezar nunca, devuelve una lista **vacía**.

### Ejemplo 1

```
n = 4, before = [[0, 2], [1, 2], [3, 1]]
Salida: [0, 3, 1, 2]
```

Al principio están listos los pasos 0 y 3; el cocinero hace el 0 y después el 3. Eso deja listo el 1, y tras el 1, el paso 2.

### Ejemplo 2

```
n = 3, before = [[0, 1], [1, 2], [2, 0]]
Salida: []
```

Cada paso espera a otro en círculo.

### Ejemplo 3

```
n = 3, before = []
Salida: [0, 1, 2]
```

### Restricciones

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ before.length ≤ 4·10⁴
- `before[i] = [a, b]` con 0 ≤ a, b < n y a ≠ b; una pareja puede repetirse
