Una ruta de senderismo de larga distancia está dividida en etapas, y `costs[i]` es lo que cuesta la etapa `i` (comida, refugios, transporte). Quieres recorrer etapas **consecutivas** sin gastar más de `budget` en total.

Devuelve la mayor cantidad de etapas consecutivas cuyo coste total no supere `budget`. Si todas las etapas son demasiado caras por sí solas, devuelve `0`.

### Ejemplo 1

```
costs = [4, 2, 1, 6, 1, 1], budget = 8
Output: 3
```

Las etapas 0 a 2 cuestan 4 + 2 + 1 = 7, y las etapas 3 a 5 cuestan 6 + 1 + 1 = 8. No caben cuatro etapas consecutivas.

### Ejemplo 2

```
costs = [9, 9, 9], budget = 5
Output: 0
```

### Ejemplo 3

```
costs = [0, 3, 0, 0], budget = 0
Output: 2
```

Las etapas que no cuestan nada siempre caben.

### Restricciones

- 1 ≤ costs.length ≤ 5·10⁴
- 0 ≤ costs[i] ≤ 10⁴
- 0 ≤ budget ≤ 10⁹
