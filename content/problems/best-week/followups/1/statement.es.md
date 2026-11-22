Esta vez la panadería anota su **beneficio** diario, `profits[i]`, y un mal día puede dar
pérdidas (un número negativo). Para el cartel puede elegir un tramo de **al menos 1 y como
mucho `k`** días consecutivos, el que quede mejor.

Devuelve el mayor beneficio total de cualquier tramo de 1 a `k` días consecutivos. Un
tramo siempre tiene al menos un día, así que si todos los días dan pérdidas, la respuesta
es la pérdida más pequeña.

### Ejemplo 1

```
profits = [3, -5, 4, 2, -1, 6], k = 3
Salida: 7
```

Los días 3 a 5 dan 2 − 1 + 6 = 7. Los días 2 a 5 darían 11, pero son cuatro días.

### Ejemplo 2

```
profits = [-4, 2, -3, 9, 8, 9], k = 2
Salida: 17
```

### Ejemplo 3

```
profits = [-12, -3, -9, -5, -6, -8], k = 1
Salida: -3
```

### Restricciones

- 1 ≤ k ≤ profits.length ≤ 2·10⁵
- −10⁴ ≤ profits[i] ≤ 10⁴
