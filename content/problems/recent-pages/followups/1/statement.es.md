La caché del lector ahora guarda las páginas que **más abres**. Las operaciones son las
mismas que antes, donde `ops[i]` es el nombre y `args[i]` los números:

- `"get"`, args `[page]`: **informa** el valor guardado de la página, o `-1` si no está en
  la caché. Una página encontrada suma un uso más.
- `"put"`, args `[page, value]`: guarda `value` para la página (sustituyendo un valor
  anterior). Una página que ya está en la caché suma un uso más; una página nueva empieza
  con un uso.

Lo que cambia es qué página sale. Si un `"put"` añade una página nueva a una caché llena,
antes se quita la página con **menos usos**; entre páginas con los mismos usos, sale la
**usada hace más tiempo**. Una página quitada olvida sus usos: si vuelve, empieza otra vez
con uno. Un `"get"` de una página ausente no cambia nada.

Devuelve los informes de las operaciones `"get"`, en orden.

### Ejemplo 1

```
capacity = 2, ops = ["put", "put", "get", "put", "get", "get"], args = [[1, 10], [2, 20], [1], [3, 30], [2], [1]]
Salida: [10, -1, 10]
```

La página 1 tiene dos usos y la página 2 solo uno, así que añadir la página 3 quita la
página 2.

### Ejemplo 2

```
capacity = 2, ops = ["get", "put", "put", "get", "get", "put"], args = [[1], [4, 483], [4, 96], [4], [4], [1, 272]]
Salida: [-1, 96, 96]
```

### Ejemplo 3

```
capacity = 2, ops = ["get", "put", "get", "put", "put", "put"], args = [[1], [2, 685], [3], [2, 595], [2, 653], [4, 880]]
Salida: [-1, -1]
```

Actualizar la página 2 no añade ninguna página, así que la página 4 cabe sin quitar nada.

### Restricciones

- 1 ≤ capacity ≤ 10⁵
- 1 ≤ ops.length ≤ 2·10⁵, args.length = ops.length
- `ops[i]` es `"get"` (args `[page]`) o `"put"` (args `[page, value]`)
- 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹
