La estantería de la bibliotecaria es la misma del problema base: carpetas con números de
catálogo distintos, ordenadas de menor a mayor, después de que un ayudante pasara un
bloque del **principio** al **final** en el mismo orden (quizá ninguna carpeta, quizá
todas menos una). `shelf` enumera los números tal como están.

Ahora llega toda una lista de peticiones. Devuelve un array cuya entrada `k` es la
posición de la carpeta con número `labels[k]`, o `-1` si no está en la estantería. Un
mismo número puede pedirse más de una vez.

### Ejemplo 1

```
shelf = [41, 57, 63, 8, 15, 22], labels = [15, 40, 41, 22]
Salida: [4, -1, 0, 5]
```

### Ejemplo 2

```
shelf = [48, 51, 54, 4, 36], labels = [4, 4, 6]
Salida: [3, 3, -1]
```

### Ejemplo 3

```
shelf = [23, 53, 5, 10], labels = [53, 37, 27]
Salida: [1, -1, -1]
```

### Restricciones

- 1 ≤ shelf.length, labels.length ≤ 10⁵
- 0 ≤ shelf[i], labels[k] ≤ 10⁹
- Los números de la estantería son todos distintos, y `shelf` es una lista ordenada con
  un bloque pasado del principio al final.

Recorrer la estantería por cada etiqueta son hasta 10¹⁰ pasos; busca O(log n) por
etiqueta.
