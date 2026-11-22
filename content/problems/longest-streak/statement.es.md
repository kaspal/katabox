Ravi apunta el código del tiempo de cada día y quiere saber cuál fue la racha más larga de días iguales. Su función funciona con casi todos los registros, pero en algunos informa de una racha más corta que la que él ve a simple vista.

Dado un arreglo de enteros `values`, devuelve la longitud de la racha más larga de elementos consecutivos iguales. Un elemento suelto es una racha de longitud 1.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
values = [3, 3, 1, 4, 4, 4]
Salida: 3
```

Los tres 4 del final forman la racha más larga.

### Ejemplo 2

```
values = [2, 5, 2, 8, 8, 8, 7]
Salida: 3
```

### Ejemplo 3

```
values = [1, 2, 1, 2, 2, 1, 1]
Salida: 2
```

### Restricciones

- 1 ≤ values.length ≤ 10⁵
- −10⁹ ≤ values[i] ≤ 10⁹
