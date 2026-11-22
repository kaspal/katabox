Esta vez algunas etapas te devuelven dinero: un **reembolso** hace que `costs[i]` sea
negativo, y el propio presupuesto también puede ser negativo (quieres terminar ganando).
Sigues queriendo recorrer etapas **consecutivas** cuyo coste total no supere `budget`.

Devuelve la mayor cantidad de etapas consecutivas (al menos una) cuyo coste total no
supere `budget`, o `0` si no existe ningún tramo así.

### Ejemplo 1

```
costs = [5, -3, 4, 6, -8, 2], budget = 4
Salida: 5
```

Las etapas 1 a 5 cuestan −3 + 4 + 6 − 8 + 2 = 1. Las seis etapas cuestan 6, demasiado.

### Ejemplo 2

```
costs = [9, -7, -1, -6, 6, 5], budget = 6
Salida: 6
```

### Ejemplo 3

```
costs = [-7, -7, 2, -4, 0, -1], budget = 10
Salida: 6
```

### Restricciones

- 1 ≤ costs.length ≤ 5·10⁴
- −10⁴ ≤ costs[i] ≤ 10⁴
- −10⁹ ≤ budget ≤ 10⁹
