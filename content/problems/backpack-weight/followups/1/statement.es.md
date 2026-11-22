La excursionista tiene ahora **varias copias** de cada objeto: el objeto `i` pesa
`weights[i]`, vale `values[i]` y hay `counts[i]` copias idénticas de él. Se puede llevar
cualquier número de copias entre `0` y `counts[i]`. La mochila sigue aguantando como mucho
`capacity` de peso total.

Devuelve el mayor valor total de las copias que caben.

### Ejemplo 1

```
weights = [3, 4], values = [30, 50], counts = [2, 1], capacity = 10
Salida: 110
```

Dos copias del objeto 0 y una del objeto 1 pesan 3 + 3 + 4 = 10 y valen 110.

### Ejemplo 2

```
weights = [5, 1, 3], values = [15, 63, 97], counts = [2, 2, 3], capacity = 9
Salida: 320
```

Las dos copias del objeto 1 y dos copias del objeto 2 pesan 8 y valen 126 + 194 = 320.

### Ejemplo 3

```
weights = [1, 1, 1], values = [46, 21, 94], counts = [3, 2, 2], capacity = 14
Salida: 368
```

Caben las 7 copias.

### Restricciones

- 1 ≤ weights.length = values.length = counts.length ≤ 50
- 1 ≤ weights[i] ≤ 10⁴
- 0 ≤ values[i] ≤ 10⁴
- 1 ≤ counts[i] ≤ 100
- 0 ≤ capacity ≤ 5000
