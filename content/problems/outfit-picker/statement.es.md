Lucía elige un conjunto para la obra del colegio en la tienda de disfraces. La tienda agrupa sus prendas por categoría (sombreros, chaquetas, zapatos, …): `prices[c]` recoge los precios de las prendas de la categoría `c`.

Un conjunto lleva **exactamente una** prenda de **cada** categoría. ¿Cuántos conjuntos distintos cuestan **como mucho** `budget` en total? Dos conjuntos son distintos si se diferencian en al menos una prenda, aunque las prendas tengan el mismo precio.

### Ejemplo 1

```
prices = [[30, 50], [20, 40, 60], [10]], budget = 100
Salida: 5
```

De los 2 · 3 · 1 = 6 conjuntos, solo 50 + 60 + 10 = 120 se pasa del presupuesto.

### Ejemplo 2

```
prices = [[500], [600]], budget = 1000
Salida: 0
```

### Ejemplo 3

```
prices = [[1, 2, 3]], budget = 2
Salida: 2
```

### Restricciones

- 1 ≤ prices.length ≤ 6
- 1 ≤ prices[c].length ≤ 6
- 1 ≤ prices[c][i] ≤ 1000
- 1 ≤ budget ≤ 6000
