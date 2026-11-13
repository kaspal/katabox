Un food truck vende unos pocos platos; `prices[i]` es el precio del plato `i`. Un cliente quiere pedir un conjunto de platos **distintos** (como mucho uno de cada) que cueste **exactamente** `budget`.

Devuelve todos esos conjuntos, cada uno escrito como la lista de sus números de plato en orden creciente. Ordena la lista de conjuntos lexicográficamente (compara los primeros elementos, luego los segundos, etc.; una lista más corta que es prefijo de otra más larga va antes). Devuelve una lista vacía si no hay ninguno.

### Ejemplo 1

```
prices = [4, 2, 6, 3], budget = 9
Salida: [[0, 1, 3], [2, 3]]
```

4 + 2 + 3 = 9 y 6 + 3 = 9.

### Ejemplo 2

```
prices = [5, 5], budget = 5
Salida: [[0], [1]]
```

### Ejemplo 3

```
prices = [7], budget = 3
Salida: []
```

### Restricciones

- 1 ≤ prices.length ≤ 15
- 1 ≤ prices[i] ≤ 1000
- 1 ≤ budget ≤ 15000
