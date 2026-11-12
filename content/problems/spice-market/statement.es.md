Un comerciante en un mercado de especias puede llevar a casa como mucho `capacity` gramos. El puesto `i` vende un saco de especia de `weights[i]` gramos por `prices[i]` monedas en total, y deja llevarse **cualquier parte** del saco: `x` gramos cuestan `prices[i] · x / weights[i]` monedas y valen exactamente eso en casa.

¿Qué especias debe llevarse, y cuánto de cada una, para llegar a casa con el **mayor valor total**? Devuelve ese valor. Se aceptan las respuestas a menos de 10⁻⁶ del valor exacto.

### Ejemplo 1

```
weights = [10, 20, 30], prices = [60, 100, 120], capacity = 50
Salida: 240
```

Por gramo, las especias valen 6, 5 y 4 monedas. El comerciante se lleva los dos primeros sacos enteros (30 gramos, 160 monedas) y 20 de los 30 gramos del tercero (80 monedas).

### Ejemplo 2

```
weights = [5], prices = [10], capacity = 100
Salida: 10
```

### Ejemplo 3

```
weights = [4, 8], prices = [10, 4], capacity = 1
Salida: 2.5
```

### Restricciones

- 1 ≤ weights.length ≤ 10⁵, prices.length = weights.length
- 1 ≤ weights[i], prices[i] ≤ 10⁴
- 1 ≤ capacity ≤ 10⁹
