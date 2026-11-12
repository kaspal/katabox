Tres primos quieren comprarle a su abuela **tres regalos distintos** de un escaparate, y les gustaría que el total quedara lo más cerca posible del dinero que juntaron, `target`. `prices[i]` es el precio del regalo `i`; dos regalos pueden costar lo mismo.

Devuelve el total de los tres regalos cuya suma está **más cerca** de `target`. Si dos totales están igual de cerca, uno por encima y otro por debajo, devuelve el **menor**.

### Ejemplo 1

```
prices = [12, 40, 7, 25, 33], target = 70
Salida: 70
```

12 + 25 + 33 = 70 da justo el objetivo.

### Ejemplo 2

```
prices = [5, 9, 14, 30], target = 20
Salida: 28
```

Los tres regalos más baratos ya cuestan 5 + 9 + 14 = 28.

### Ejemplo 3

```
prices = [10, 20, 30, 40], target = 75
Salida: 70
```

70 (10 + 20 + 40) y 80 (10 + 30 + 40) están ambos a 5; gana el menor.

### Restricciones

- 3 ≤ prices.length ≤ 2000
- 1 ≤ prices[i] ≤ 10⁶
- 1 ≤ target ≤ 3·10⁶
