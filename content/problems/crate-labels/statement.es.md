Un almacén recibe cajas mixtas de fruta. Cada caja lleva una etiqueta con su contenido, por ejemplo `apples:12;pears:3`: nombres de productos con sus cantidades, separados por punto y coma. El encargado del inventario quiere un total por producto sumando todas las cajas.

Este es un **problema de programa**: lee las etiquetas de la entrada estándar y escribe los totales en la salida estándar.

## Formato de entrada

- La primera línea contiene `N`, el número de cajas.
- Después, `N` líneas, una etiqueta cada una. Una etiqueta es una o más entradas `name:quantity` separadas por `;`, sin espacios. El mismo producto puede aparecer en muchas etiquetas, e incluso dos veces en la misma.

## Formato de salida

Escribe una línea por cada producto que aparezca en alguna etiqueta: `name total`, separados por un solo espacio, donde `total` es la suma de todas sus cantidades. Ordena los productos alfabéticamente, comparando letra a letra; un nombre que es el comienzo de otro va primero (`fig` antes que `figs`).

### Ejemplo 1

Entrada:

```
3
apples:12;pears:3
pears:5;apples:1;figs:40
kiwis:7
```

Salida:

```
apples 13
figs 40
kiwis 7
pears 8
```

Las manzanas (`apples`) suman 12 + 1 = 13 y las peras (`pears`), 3 + 5 = 8.

### Ejemplo 2

Entrada:

```
1
plums:1000
```

Salida:

```
plums 1000
```

### Ejemplo 3

Entrada:

```
2
fig:2;figs:3;fig:4
figs:1
```

Salida:

```
fig 6
figs 4
```

`fig` aparece dos veces en la primera etiqueta: 2 + 4 = 6.

### Restricciones

- 1 ≤ N ≤ 1000
- Una etiqueta tiene de 1 a 20 entradas
- Un nombre tiene de 1 a 10 letras minúsculas (`a`-`z`)
- 1 ≤ quantity ≤ 1000
