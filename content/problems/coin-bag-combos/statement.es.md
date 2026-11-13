Una máquina expendedora acepta monedas de unos pocos valores, `coins` (todos distintos), y puedes usar cualquier cantidad de monedas de cada valor. Enumera todas las formas de pagar exactamente `target`.

Una forma de pagar es una bolsa de monedas: el orden no importa, así que escribe cada una con sus valores en orden no decreciente. Devuelve todas las formas, ordenadas lexicográficamente (se comparan las primeras monedas, después las segundas, y así sucesivamente). Si no hay ninguna forma, devuelve una lista vacía.

### Ejemplo 1

```
coins = [3, 4, 7], target = 10
Salida: [[3, 3, 4], [3, 7]]
```

### Ejemplo 2

```
coins = [4], target = 6
Salida: []
```

### Ejemplo 3

```
coins = [7, 1], target = 3
Salida: [[1, 1, 1]]
```

### Restricciones

- 1 ≤ coins.length ≤ 10, todos los valores distintos
- 1 ≤ coins[i] ≤ 50
- 1 ≤ target ≤ 60
- Hay como mucho 1000 formas de pagar.
