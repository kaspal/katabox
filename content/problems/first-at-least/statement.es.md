Hana guarda los pesos de los pedidos de su panadería en una lista ordenada, y una búsqueda binaria le dice dónde empiezan los pedidos que llegan a cierto peso. Responde a casi todo al instante, pero con algunos pesos señala el sitio equivocado.

Dado un arreglo `sorted` en orden no decreciente y un entero `x`, devuelve el menor índice `i` tal que `sorted[i] ≥ x`. Si ningún elemento llega a `x`, devuelve `sorted.length`.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
sorted = [2, 5, 5, 9], x = 5
Salida: 1
```

El índice 1 contiene el primer 5.

### Ejemplo 2

```
sorted = [-16, -13, -4, 11, 16], x = 8
Salida: 3
```

### Ejemplo 3

```
sorted = [-15, -15, -10, -1, 3], x = 5
Salida: 5
```

Todos los elementos son menores que 5, así que la respuesta es la longitud del arreglo.

### Restricciones

- 1 ≤ sorted.length ≤ 10⁵
- −10⁹ ≤ sorted[i], x ≤ 10⁹
- sorted está en orden no decreciente.
