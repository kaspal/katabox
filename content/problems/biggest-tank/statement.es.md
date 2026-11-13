Una valla es una fila de tablas verticales separadas un metro; `boards[i]` es la altura de la tabla `i`. Un granjero quiere construir un depósito de agua de lluvia eligiendo **dos** tablas y cerrando el espacio entre ellas (las tablas de en medio se quitan). La anchura del depósito es la distancia entre las dos tablas, y el agua solo puede subir hasta la altura de la **más baja** de las dos.

Devuelve la mayor cantidad de agua que puede contener un depósito: `(j − i) · min(boards[i], boards[j])` para el mejor par `i < j`.

### Ejemplo 1

```
boards = [2, 7, 4, 1, 8, 3, 6]
Salida: 30
```

Las tablas 1 y 6: anchura 5, altura min(7, 6) = 6.

### Ejemplo 2

```
boards = [5, 5]
Salida: 5
```

### Ejemplo 3

```
boards = [0, 3, 0]
Salida: 0
```

### Ejemplo 4

```
boards = [1, 9, 9, 1]
Salida: 9
```

### Restricciones

- 2 ≤ boards.length ≤ 2·10⁵
- 0 ≤ boards[i] ≤ 10⁴
