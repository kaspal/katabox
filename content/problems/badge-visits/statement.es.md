Un museo de ciencias entrega a cada socio una credencial con un número de identificación. El torniquete de la entrada la escanea cada vez que alguien entra, y un mismo socio puede volver varias veces en el día. Los escaneos del día se guardan en orden. La directora siempre hace el mismo tipo de pregunta: «después de los primeros `p` escaneos del día, ¿cuántos socios **distintos** habían entrado?».

Este es un **problema de programa**: lee los escaneos y las preguntas de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `N`, el número de escaneos.
- La segunda línea contiene los `N` identificadores en el orden en que se escanearon, separados por espacios.
- La tercera línea contiene `Q`, el número de preguntas.
- Cada una de las `Q` líneas siguientes contiene un número `p`.

## Formato de salida

Escribe `Q` líneas, una por pregunta y en orden: cuántos identificadores distintos hay entre los primeros `p` escaneos.

### Ejemplo 1

Entrada:

```
6
5 3 5 9 3 1
4
1
3
4
6
```

Salida:

```
1
2
3
4
```

Tras 3 escaneos el torniquete ha visto las credenciales 5 y 3 (la 5 dos veces), así que la respuesta es 2.

### Ejemplo 2

Entrada:

```
4
7 7 7 7
3
4
2
4
```

Salida:

```
1
1
1
```

### Ejemplo 3

Entrada:

```
5
1000000000 1 999999999 1 1000000000
5
5
2
3
1
5
```

Salida:

```
3
2
3
1
3
```

La misma pregunta puede repetirse, y las preguntas llegan en cualquier orden.

### Restricciones

- 1 ≤ N ≤ 30 000
- 1 ≤ identificador ≤ 10⁹
- 1 ≤ Q ≤ 30 000
- 1 ≤ p ≤ N
