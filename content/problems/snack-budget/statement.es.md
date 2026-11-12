Una monitora de excursiones tiene `budget` céntimos para comprar meriendas. La tienda tiene una unidad de cada merienda, y `prices[i]` es el precio de la merienda `i` en céntimos.

Devuelve el **mayor número de meriendas** que puede comprar sin gastar más de `budget`.

### Ejemplo 1

```
prices = [120, 35, 80, 60, 200], budget = 180
Salida: 3
```

35 + 60 + 80 = 175 cabe. No hay cuatro meriendas que cuesten 180 o menos.

### Ejemplo 2

```
prices = [50, 50, 50], budget = 49
Salida: 0
```

### Ejemplo 3

```
prices = [10, 10, 10, 10], budget = 40
Salida: 4
```

### Restricciones

- 1 ≤ prices.length ≤ 10⁵
- 1 ≤ prices[i] ≤ 10⁴
- 0 ≤ budget ≤ 10⁹
