Un sitio de noticias guarda sus páginas en caché, conservando las **populares**. La caché tiene como mucho `capacity` páginas, y cada página guardada tiene un **recuento de usos**. Procesa las operaciones en orden, donde `ops[i]` es el nombre y `args[i]` los números:

- `"get"`, args `[page]`: **informa** el valor guardado de la página, o `-1` si no está en la caché. Una página encontrada suma un uso.
- `"put"`, args `[page, value]`: guarda `value` para la página. Si la página está en caché, su valor se sustituye y suma un uso. Si es nueva, entra con un recuento de 1; pero antes, si la caché está llena, se quita la página con el **menor recuento de usos** y, entre esas, la **usada hace más tiempo**.

Devuelve los informes de las operaciones `"get"`, en orden.

### Ejemplo 1

```
capacity = 2, ops = ["put", "put", "get", "put", "get", "get", "put", "get", "get"], args = [[3, 30], [4, 40], [3], [5, 50], [4], [5], [6, 60], [3], [6]]
Salida: [30, -1, 50, -1, 60]
```

La página 3 tiene 2 usos, así que la página 5 sustituye a la 4. Después, las páginas 3 y 5 tienen 2 usos cada una; la 3 se usó hace más tiempo, así que la página 6 la sustituye.

### Ejemplo 2

```
capacity = 1, ops = ["put", "get", "put", "get", "get"], args = [[1, 5], [1], [2, 6], [1], [2]]
Salida: [5, -1, 6]
```

### Ejemplo 3

```
capacity = 2, ops = ["put", "put", "put", "put", "get", "get"], args = [[1, 1], [1, 2], [2, 3], [3, 4], [1], [2]]
Salida: [2, -1]
```

Actualizar la página 1 le da 2 usos, así que la página 3 sustituye a la 2.

### Restricciones

- 1 ≤ capacity ≤ 10⁵
- 1 ≤ ops.length ≤ 2·10⁵, args.length = ops.length
- 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹
