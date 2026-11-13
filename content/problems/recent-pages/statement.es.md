Un lector de libros electrónicos guarda las páginas que abriste hace poco en una pequeña caché de `capacity` páginas. Procesa las operaciones en orden, donde `ops[i]` es el nombre y `args[i]` los números:

- `"get"`, args `[page]`: **informa** el valor guardado de la página, o `-1` si no está en la caché. Una página encontrada cuenta como usada ahora.
- `"put"`, args `[page, value]`: guarda `value` para la página (sustituyendo un valor anterior) y cuenta la página como usada ahora.

Si un `"put"` añade una página nueva a una caché llena, antes se quita la página **usada hace más tiempo**.

Devuelve los informes de las operaciones `"get"`, en orden.

### Ejemplo 1

```
capacity = 2, ops = ["put", "get", "put", "put", "put", "get", "get"], args = [[5, 50], [5], [8, 80], [5, 55], [9, 90], [8], [5]]
Salida: [50, -1, 55]
```

Actualizar la página 5 la convierte en la más reciente, así que añadir la página 9 quita la página 8.

### Ejemplo 2

```
capacity = 1, ops = ["put", "put", "get", "get"], args = [[1, 7], [2, 3], [1], [2]]
Salida: [-1, 3]
```

### Ejemplo 3

```
capacity = 3, ops = ["get"], args = [[4]]
Salida: [-1]
```

### Restricciones

- 1 ≤ capacity ≤ 10⁵
- 1 ≤ ops.length ≤ 2·10⁵, args.length = ops.length
- 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹
