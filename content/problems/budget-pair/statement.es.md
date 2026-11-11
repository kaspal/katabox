Una tienda de regalos publica sus precios `prices` en orden no decreciente. Quieres comprar exactamente **dos artículos distintos** (dos posiciones diferentes de la lista) y gastar todo lo posible de tu presupuesto `budget` sin pasarte.

Devuelve el mayor total de dos artículos distintos que no supere `budget`, o `-1` si no hay dos artículos que quepan.

### Ejemplo 1

```
prices = [3, 5, 8, 12, 20], budget = 19
Salida: 17
```

5 + 12 = 17 es el mejor total que no pasa de 19.

### Ejemplo 2

```
prices = [4, 4, 9], budget = 8
Salida: 8
```

Los dos artículos de precio 4 son artículos distintos.

### Ejemplo 3

```
prices = [10, 15], budget = 24
Salida: -1
```

### Restricciones

- 2 ≤ prices.length ≤ 5·10⁴
- 1 ≤ prices[i] ≤ 10⁶
- prices está ordenado de forma no decreciente.
- 1 ≤ budget ≤ 2·10⁶
