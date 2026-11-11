Una estación meteorológica registra la lluvia de cada día en milímetros: `rain[d]` es la lluvia del día `d` (los días se numeran desde 0).

Los agricultores envían consultas `queries`, cada una `[first, last]`, preguntando cuánta lluvia cayó desde el día `first` hasta el día `last`, ambos incluidos. Devuelve las respuestas en el orden de las consultas.

### Ejemplo 1

```
rain = [3, 0, 7, 2, 5], queries = [[0, 2], [1, 3], [4, 4]]
Salida: [10, 9, 5]
```

Entre los días 0 y 2 cayeron 3 + 0 + 7 = 10 mm.

### Ejemplo 2

```
rain = [0, 0, 0], queries = [[0, 2]]
Salida: [0]
```

### Restricciones

- 1 ≤ rain.length ≤ 10⁵
- 0 ≤ rain[d] ≤ 10⁴
- 1 ≤ queries.length ≤ 10⁵
- queries[j] = [first, last] con 0 ≤ first ≤ last < rain.length
