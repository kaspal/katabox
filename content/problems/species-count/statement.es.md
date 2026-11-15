Una cámara de sendero en una reserva natural toma `n` fotos, y para cada una los guardas anotan el identificador de la especie que aparece. Después estudian tramos del registro: ¿cuántas especies distintas aparecen entre las fotos `l` y `r`? El registro está cerrado y no cambia, pero los guardas hacen muchísimas preguntas.

Este es un **problema de programa**: lee el registro y las preguntas de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `n`, el número de fotos.
- La segunda línea contiene los `n` identificadores de especie, separados por espacios, en el orden en que se tomaron las fotos (primero la foto 1).
- La tercera línea contiene `Q`, el número de preguntas.
- Después, `Q` líneas, cada una `l r`: el tramo desde la foto `l` hasta la foto `r`, ambas incluidas.

## Formato de salida

Escribe `Q` líneas, una por pregunta y en orden: el número de identificadores de especie distintos entre las fotos `l` y `r`.

### Ejemplo 1

Entrada:

```
7
12 5 12 9 5 5 31
4
1 3
2 6
4 4
1 7
```

Salida:

```
2
3
1
4
```

Las fotos 2 a 6 muestran las especies 5, 12, 9, 5 y 5: tres especies distintas.

### Ejemplo 2

Entrada:

```
1
1000000000
1
1 1
```

Salida:

```
1
```

### Ejemplo 3

Entrada:

```
5
3 3 3 3 3
3
1 5
2 3
5 5
```

Salida:

```
1
1
1
```

### Restricciones

- 1 ≤ n ≤ 5·10⁴
- 1 ≤ id ≤ 10⁹, escrito sin ceros a la izquierda
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ l ≤ r ≤ n
