Tomás lleva un registro del tiempo para su huerto y escribió un pequeño script que responde preguntas como «¿cuánta lluvia cayó del día 3 al día 9?». Al principio los totales parecían razonables, pero su vecina, que apunta la lluvia en papel, siempre encuentra que se quedan cortos.

`rain[i]` es la lluvia caída el día `i` (los días se numeran desde 0). Cada elemento de `queries` es un par `[l, r]`. Devuelve una lista con un número por consulta, en el mismo orden: la lluvia total desde el día `l` hasta el día `r`, ambos incluidos.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
rain = [3, 0, 5, 2, 7], queries = [[0, 2], [1, 3], [4, 4]]
Salida: [8, 7, 7]
```

Los días 0 a 2 suman 3 + 0 + 5 = 8; los días 1 a 3 suman 0 + 5 + 2 = 7; el día 4 solo suma 7.

### Ejemplo 2

```
rain = [4,1,7,7], queries = [[3,3],[1,1]]
Salida: [7,1]
```

### Ejemplo 3

```
rain = [1,5,2,4], queries = [[2,2],[0,1]]
Salida: [2,6]
```

### Restricciones

- 1 ≤ rain.length ≤ 10⁵
- 0 ≤ rain[i] ≤ 1000
- 1 ≤ queries.length ≤ 10⁴
- Cada consulta es `[l, r]` con 0 ≤ l ≤ r < rain.length
