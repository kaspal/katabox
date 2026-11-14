Un mareógrafo en la bocana del puerto ha registrado el nivel del agua `N` veces, a intervalos regulares, en centímetros por encima o por debajo del cero de la carta náutica. Los prácticos que preparan la entrada de un barco de mucho calado estudian tramos antiguos del registro: entre las lecturas `l` y `r`, ¿hasta dónde bajó el agua? El registro está cerrado y no cambia, pero los prácticos hacen muchísimas preguntas.

Este es un **problema de programa**: lee el registro y las preguntas de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `N`, el número de lecturas.
- La segunda línea contiene los `N` niveles, separados por espacios, en el orden en que se registraron (primero la lectura 1).
- La tercera línea contiene `Q`, el número de preguntas.
- Después, `Q` líneas, cada una `l r`: el tramo desde la lectura `l` hasta la lectura `r`, ambas incluidas.

## Formato de salida

Escribe `Q` líneas, una por pregunta y en orden: el nivel más bajo entre las lecturas `l` y `r`.

### Ejemplo 1

Entrada:

```
6
12 -3 7 0 -8 5
4
1 3
4 4
4 6
1 6
```

Salida:

```
-3
0
-8
-8
```

Las lecturas 4 a 6 son 0, -8 y 5; la más baja es -8.

### Ejemplo 2

Entrada:

```
1
-1000000
1
1 1
```

Salida:

```
-1000000
```

### Ejemplo 3

Entrada:

```
5
4 4 2 2 9
3
1 2
2 4
5 5
```

Salida:

```
4
2
9
```

### Restricciones

- 1 ≤ N ≤ 10⁵
- -10⁶ ≤ nivel ≤ 10⁶
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ l ≤ r ≤ N
