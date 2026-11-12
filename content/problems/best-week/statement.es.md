Una panadería anotó cuántas barras vendió cada día, en orden, en `sales`. Para el cartel de su aniversario quiere presumir de su mejor racha de `k` días **consecutivos**.

Devuelve el mayor total de barras vendidas en cualquier tramo de `k` días consecutivos.

### Ejemplo 1

```
sales = [12, 30, 7, 25, 18, 4], k = 3
Salida: 62
```

Los días 1 a 3 vendieron 30 + 7 + 25 = 62; ningún otro tramo de tres días vendió más.

### Ejemplo 2

```
sales = [9, 0, 9], k = 1
Salida: 9
```

### Ejemplo 3

```
sales = [5, 6, 2], k = 3
Salida: 13
```

Si `k` es el número de días, el único tramo es el registro completo.

### Restricciones

- 1 ≤ k ≤ sales.length ≤ 2·10⁵
- 0 ≤ sales[i] ≤ 10⁴
