La pared de un edificio se dibuja como una cuadrícula de casillas: `.` es una casilla libre y `#` una bloqueada (una ventana, una tubería, un letrero). Los dueños quieren colgar un cartel rectangular, alineado con la cuadrícula, que cubra solo casillas libres. ¿Qué tamaño puede tener como mucho?

Este es un **problema de programa**: lee la pared de la entrada estándar y escribe el área más grande en la salida estándar.

## Formato de entrada

- La primera línea contiene `R C`: el número de filas y de columnas de la pared.
- Después, `R` líneas, cada una con exactamente `C` caracteres, `.` o `#`, desde la fila de arriba hacia abajo.

## Formato de salida

Escribe un número: la mayor cantidad de casillas que puede cubrir un cartel, o `0` si no hay ninguna casilla libre.

### Ejemplo 1

Entrada:

```
4 5
.#...
.....
##...
.#..#
```

Salida:

```
9
```

Las tres últimas columnas de las tres primeras filas están libres: un cartel de 3 × 3.

### Ejemplo 2

Entrada:

```
2 3
###
###
```

Salida:

```
0
```

Todas las casillas están bloqueadas.

### Ejemplo 3

Entrada:

```
3 4
....
.#..
....
```

Salida:

```
6
```

Las dos últimas columnas están libres de arriba abajo: 3 × 2 = 6, más que cualquier fila completa (4).

### Restricciones

- 1 ≤ R, C ≤ 1500
- Cada fila tiene exactamente `C` caracteres, `.` o `#`
