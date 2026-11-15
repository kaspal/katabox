A un rotulista le han pedido que convierta el nombre de una tienda en un palíndromo, una palabra que se lee igual desde los dos extremos. El nombre ya está pintado en el cartel y no se puede tocar, y solo queda espacio libre a su izquierda, así que el rotulista solo puede añadir letras delante. La pintura es cara: cuantas menos letras, mejor.

Este es un **problema de programa**: lee la palabra de la entrada estándar y escribe el palíndromo más corto que termina con ella en la salida estándar.

## Formato de entrada

- Una línea contiene la palabra `s`, en letras minúsculas.

## Formato de salida

Escribe una línea: el palíndromo más corto formado por algunas letras seguidas de `s`. Si `s` ya es un palíndromo, escríbela sin cambios.

### Ejemplo 1

Entrada:

```
abac
```

Salida:

```
cabac
```

`abac` empieza con el palíndromo `aba`, así que basta con una `c` delante. Ninguna palabra más corta sirve: `abac` por sí sola no es un palíndromo.

### Ejemplo 2

Entrada:

```
racecar
```

Salida:

```
racecar
```

### Ejemplo 3

Entrada:

```
ab
```

Salida:

```
bab
```

### Restricciones

- 1 ≤ |s| ≤ 2·10⁵
- `s` contiene solo las letras minúsculas de la `a` a la `z`.
