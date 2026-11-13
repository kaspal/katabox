Un coleccionista sigue el precio de una cámara antigua en un mercado de segunda mano, un precio por día, en `prices`. Una **caída brusca** es un par de días, uno anterior `i` y otro posterior `j` (`i < j`, no necesariamente seguidos), en el que el precio posterior es menos de la mitad del anterior: `prices[i] > 2 · prices[j]`.

Devuelve el número de caídas bruscas.

### Ejemplo 1

```
prices = [9, 4, 3, 10, 2]
Salida: 4
```

9 → 4, 9 → 3, 9 → 2 y 10 → 2. El par 4 → 2 es justo la mitad, que no es menos de la mitad.

### Ejemplo 2

```
prices = [1, 2, 3]
Salida: 0
```

### Ejemplo 3

```
prices = [5, 2, 2]
Salida: 2
```

### Restricciones

- 1 ≤ prices.length ≤ 2·10⁵
- 1 ≤ prices[i] ≤ 10⁹
