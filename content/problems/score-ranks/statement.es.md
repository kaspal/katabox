Una escuela de idiomas hace un examen de nivel a todos sus alumnos nuevos. Para no crear tensiones, no ordena a los alumnos uno por uno: ordena las **puntuaciones**. La mejor puntuación que sacó alguien es el puesto 1, la siguiente puntuación más baja que sacó alguien es el puesto 2, y así sucesivamente; los alumnos con la misma puntuación comparten puesto.

Después, los alumnos preguntan en recepción: «¿qué puesto corresponde a una puntuación de `x`?». Si nadie sacó exactamente `x`, la respuesta es `0`.

Este es un **problema de programa**: lee las puntuaciones y las preguntas de la entrada estándar y escribe los puestos en la salida estándar.

## Formato de entrada

- La primera línea contiene `N`, el número de alumnos.
- La segunda línea contiene las `N` puntuaciones, separadas por espacios.
- La tercera línea contiene `Q`, el número de preguntas.
- La cuarta línea contiene las `Q` puntuaciones preguntadas `x`, separadas por espacios.

## Formato de salida

Escribe **una sola línea** con `Q` números separados por un espacio: para cada puntuación preguntada, en orden, su puesto entre las puntuaciones distintas (1 = la más alta), o `0` si ningún alumno sacó exactamente esa puntuación.

### Ejemplo 1

Entrada:

```
6
70 85 70 100 60 85
5
85 70 100 90 60
```

Salida:

```
2 3 1 0 4
```

Las puntuaciones distintas, de mayor a menor, son 100, 85, 70 y 60. Nadie sacó 90.

### Ejemplo 2

Entrada:

```
3
0 0 0
3
0 1 1000000000
```

Salida:

```
1 0 0
```

### Ejemplo 3

Entrada:

```
5
5 4 3 2 1
4
1 5 3 6
```

Salida:

```
5 1 3 0
```

### Restricciones

- 1 ≤ N ≤ 20 000
- 0 ≤ puntuación ≤ 10⁹
- 1 ≤ Q ≤ 20 000
- 0 ≤ x ≤ 10⁹
