Un estadio imprime un número de entrada en cada asiento. En su plano `seats`, los números crecen estrictamente de izquierda a derecha en cada fila y estrictamente de arriba abajo en cada columna (pero una fila no tiene por qué empezar después de donde acaba la anterior).

Devuelve si algún asiento tiene el número `ticket`.

### Ejemplo 1

```
seats = [[2, 5, 9],
         [4, 8, 12],
         [7, 11, 15]],
ticket = 11
Salida: true
```

### Ejemplo 2

```
seats = [[2, 5, 9], [4, 8, 12], [7, 11, 15]], ticket = 6
Salida: false
```

### Ejemplo 3

```
seats = [[10]], ticket = 10
Salida: true
```

### Restricciones

- 1 ≤ filas, columnas ≤ 300 (todas las filas tienen la misma longitud)
- 0 ≤ seats[r][c], ticket ≤ 10⁹
- Las filas y las columnas son estrictamente crecientes.

Un recorrido completo también pasa las pruebas; el ejercicio es una búsqueda en O(filas · log columnas) u O(filas + columnas).
