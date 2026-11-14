Una guardabosques sobrevuela con un dron una ladera recién replantada y convierte la foto en un mapa de casillas: `#` es un árbol joven y `.` es hierba. Ya en la caseta, las cuadrillas de plantación la llaman con preguntas como «¿cuántos árboles hay en la fila 12?» o «¿y en la columna 40?», y ella quiere contestar cada una al momento.

Este es un **problema de programa**: lee el mapa y las preguntas de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `R C`, el número de filas y de columnas del mapa.
- Las `R` líneas siguientes son las filas del mapa, de arriba abajo: cada una tiene exactamente `C` caracteres, todos `#` o `.`, sin espacios.
- La línea siguiente contiene `Q`, el número de preguntas.
- Cada una de las `Q` líneas siguientes es una pregunta: `row r` o `col c`, con índices desde 0 (la fila 0 es la de arriba y la columna 0 la de la izquierda).

## Formato de salida

Escribe `Q` líneas, una por pregunta y en orden: el número de árboles de esa fila o columna.

### Ejemplo 1

Entrada:

```
3 4
#..#
....
.###
4
row 0
row 1
col 3
col 0
```

Salida:

```
2
0
2
1
```

La columna 3 es la de más a la derecha: `#`, `.`, `#`.

### Ejemplo 2

Entrada:

```
1 1
#
2
row 0
col 0
```

Salida:

```
1
1
```

### Ejemplo 3

Entrada:

```
2 5
.....
#.#.#
3
col 4
row 1
col 1
```

Salida:

```
1
3
0
```

### Restricciones

- 1 ≤ R, C ≤ 300
- 1 ≤ Q ≤ 2·10⁴
- 0 ≤ r < R y 0 ≤ c < C
