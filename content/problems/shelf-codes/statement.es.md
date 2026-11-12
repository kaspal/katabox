Una tienda de discos guarda sus números de catálogo en una lista ordenada de menor a mayor; un número puede aparecer más de una vez (varias copias del mismo disco). Los clientes preguntan a menudo "¿cuántos discos están archivados antes del número `x`?", es decir, cuántos números de catálogo son **estrictamente menores** que `x`.

`codes` es la lista ordenada y `queries` contiene los números de los clientes. Devuelve la respuesta de cada consulta, en orden.

### Ejemplo 1

```
codes = [3, 8, 8, 15, 20], queries = [8, 1, 21, 16]
Salida: [1, 0, 5, 4]
```

Solo 3 es menor que 8. Ninguno es menor que 1, todos son menores que 21 y cuatro números son menores que 16.

### Ejemplo 2

```
codes = [5], queries = [5, 6]
Salida: [0, 1]
```

### Ejemplo 3

```
codes = [2, 2, 2], queries = [2, 3, 0]
Salida: [0, 3, 0]
```

### Restricciones

- 1 ≤ codes.length, queries.length ≤ 10⁵
- 0 ≤ codes[i], queries[j] ≤ 10⁹
- `codes` está ordenada de forma no decreciente
