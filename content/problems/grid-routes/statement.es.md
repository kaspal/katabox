Ingrid programa el robot de reparto de un almacén pequeño y quiere saber cuántas rutas distintas puede seguir desde el muelle de carga hasta la mesa de embalaje. Añadió una memoria a su función de conteo para que fuera rápida y, desde entonces, algunos planos devuelven totales imposibles.

El suelo del almacén `floor` es una cuadrícula de filas de igual longitud, donde `.` es una celda libre y `#` es una estantería. El robot empieza en la celda de arriba a la izquierda y debe llegar a la de abajo a la derecha, moviéndose una celda a la derecha o una hacia abajo cada vez y sin entrar nunca en una estantería. Devuelve el número de rutas distintas módulo 1 000 000 007 (10⁹ + 7); si la celda de salida o la de llegada es una estantería, la respuesta es 0.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
floor = ["...", ".#.", "..."]
Salida: 2
```

El robot rodea la estantería del centro, ya sea por los bordes superior y derecho o por los bordes izquierdo e inferior.

### Ejemplo 2

```
floor = ["...","..#"]
Salida: 0
```

La mesa de embalaje está tapada por una estantería.

### Ejemplo 3

```
floor = ["#..","..."]
Salida: 0
```

### Restricciones

- 1 ≤ floor.length ≤ 100
- 1 ≤ floor[i].length ≤ 100, y todas las filas tienen la misma longitud.
- Cada carácter es `.` o `#`.
