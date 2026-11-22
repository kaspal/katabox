Oliver prepara una promoción de dos por uno en su librería y quiere saber cuántas combinaciones de precios distintas suman exactamente el total de la oferta. Su script funciona con un catálogo ordenadito, pero cuando varios libros comparten precio cuenta la misma combinación más de una vez.

`prices` está ordenado de forma no decreciente. Una pareja de valores `a ≤ b` cuenta si dos libros en posiciones distintas tienen precios `a` y `b` y `a + b = budget`. Devuelve cuántas parejas así hay, contando cada pareja de valores una sola vez, sin importar cuántos libros compartan esos precios.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
prices = [1, 1, 2, 3, 3, 3], budget = 4
Salida: 1
```

La única pareja de valores es 1 + 3; 2 + 2 necesitaría dos libros de precio 2.

### Ejemplo 2

```
prices = [1,1,3,4,5], budget = 8
Salida: 1
```

### Ejemplo 3

```
prices = [1,1,2,3,6], budget = 6
Salida: 0
```

### Restricciones

- 1 ≤ prices.length ≤ 10⁵
- 1 ≤ prices[i] ≤ 10⁴
- prices está ordenado de forma no decreciente.
- 2 ≤ budget ≤ 2·10⁴
