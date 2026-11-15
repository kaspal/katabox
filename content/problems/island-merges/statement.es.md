Una ingeniera está ganando tierra a un mar poco profundo, cartografiado como una cuadrícula de `R` filas y `C` columnas de casillas. Al principio todo es agua. Cada día su equipo rellena una casilla con tierra, y los planificadores quieren saber cuántas islas hay después de cada relleno. Dos casillas de tierra pertenecen a la misma isla cuando comparten un lado; tocarse solo por una esquina no las une. A veces el equipo rellena una casilla que ya es tierra, y eso no cambia nada.

Este es un **problema de programa**: lee el tamaño del mapa y las casillas rellenadas de la entrada estándar y escribe el número de islas después de cada una en la salida estándar.

## Formato de entrada

- La primera línea contiene `R C Q`: el número de filas, el número de columnas y el número de casillas rellenadas.
- Después, `Q` líneas, cada una `r c`: la fila y la columna de la casilla rellenada ese día, en orden.

## Formato de salida

Escribe `Q` líneas: después de cada casilla rellenada, en orden, el número de islas del mapa.

### Ejemplo 1

Entrada:

```
3 3 5
1 1
1 3
2 2
1 2
3 3
```

Salida:

```
1
2
3
1
2
```

Las casillas (1, 1), (1, 3) y (2, 2) solo se tocan por las esquinas: tres islas. Rellenar (1, 2) une las tres en una. (3, 3) toca a (2, 2) solo por una esquina, así que empieza una isla nueva.

### Ejemplo 2

Entrada:

```
1 1 2
1 1
1 1
```

Salida:

```
1
1
```

La misma casilla se rellena dos veces: el segundo relleno no cambia nada.

### Ejemplo 3

Entrada:

```
2 4 4
1 1
2 4
1 4
2 1
```

Salida:

```
1
2
2
2
```

### Restricciones

- 1 ≤ R, C ≤ 1000
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ r ≤ R
- 1 ≤ c ≤ C
