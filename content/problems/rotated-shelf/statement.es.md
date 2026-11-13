Una bibliotecaria tiene una estantería de carpetas con números de catálogo distintos, ordenadas de menor a mayor. Una mañana, un ayudante cogió algunas carpetas del **principio** de la estantería y las puso, en el mismo orden, al **final** (quizá ninguna, quizá todas menos una). Ahora `shelf` enumera los números tal como están.

Devuelve la posición de la carpeta con número `label`, o `-1` si no está en la estantería.

### Ejemplo 1

```
shelf = [41, 57, 63, 8, 15, 22], label = 15
Salida: 4
```

La estantería era `[8, 15, 22, 41, 57, 63]` y las tres primeras carpetas se pasaron al final.

### Ejemplo 2

```
shelf = [41, 57, 63, 8, 15, 22], label = 40
Salida: -1
```

### Ejemplo 3

```
shelf = [5], label = 5
Salida: 0
```

### Restricciones

- 1 ≤ shelf.length ≤ 10⁵
- 0 ≤ shelf[i], label ≤ 10⁹
- Los números son todos distintos, y `shelf` es una lista ordenada con un bloque pasado del principio al final.

Un recorrido O(n) también pasa las pruebas; el ejercicio es la búsqueda O(log n).
