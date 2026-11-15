Una directora de coro coloca a los cantantes de cada sección en la tarima. Para cada sección quiere saber la altura mediana: se ordena a los cantantes del más bajo al más alto y se toma el del medio. Cuando una sección tiene un número par de cantantes hay dos en el medio, y ella elige el más bajo de los dos.

Este es un **problema de programa**: lee las secciones de la entrada estándar y escribe sus alturas medianas en la salida estándar.

## Formato de entrada

La primera línea contiene `T`, el número de secciones. Después, cada sección ocupa dos líneas:

- una línea con `n`, el número de cantantes de la sección;
- una línea con las `n` alturas, separadas por espacios, sin ningún orden concreto.

## Formato de salida

Escribe `T` líneas, una por sección y en orden: la altura mediana de esa sección.

### Ejemplo 1

Entrada:

```
3
5
170 162 181 158 175
4
160 190 150 170
1
203
```

Salida:

```
170
160
203
```

Ordenada por altura, la primera sección es 158, 162, 170, 175, 181, así que el cantante del medio mide 170. La segunda es 150, 160, 170, 190: los dos del medio son 160 y 170, y el más bajo es 160.

### Ejemplo 2

Entrada:

```
1
2
1000000000 1
```

Salida:

```
1
```

### Ejemplo 3

Entrada:

```
2
6
5 5 5 5 5 5
3
9 1 5
```

Salida:

```
5
5
```

### Restricciones

- 1 ≤ T ≤ 1000
- 1 ≤ n
- Σn ≤ 10⁵ (todas las secciones juntas)
- 1 ≤ altura ≤ 10⁹
