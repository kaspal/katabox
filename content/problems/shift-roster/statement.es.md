El banco de alimentos del barrio funciona gracias a sus voluntarios. Al final de cada mes, la coordinadora pasa el registro de turnos a un breve informe: el total de horas que el almacén estuvo atendido y, a continuación, los voluntarios que dedicaron más tiempo que la media, para darles las gracias en el boletín.

Este es un **problema de programa**: lee el registro de la entrada estándar y escribe el informe en la salida estándar.

## Formato de entrada

La primera línea contiene `N`, el número de voluntarios. Cada una de las `N` líneas siguientes contiene el nombre de un voluntario y las horas que trabajó este mes, separados por un espacio: `nombre horas`.

## Formato de salida

- La primera línea es el total de todas las horas.
- Después, en el orden de la entrada, una línea `nombre horas` por cada voluntario cuyas horas sean **estrictamente mayores** que la media (el total dividido entre `N`).
- Si ningún voluntario supera la media, la segunda línea es la palabra `none`.

Las líneas se comparan exactamente, salvo los espacios al final: usa un solo espacio entre el nombre y las horas, y no escribas líneas en blanco.

### Ejemplo 1

Entrada:

```
4
ana 8
bruno 6
chen 10
dora 4
```

Salida:

```
28
ana 8
chen 10
```

La media es 28 / 4 = 7.

### Ejemplo 2

Entrada:

```
3
eli 5
fay 5
gus 5
```

Salida:

```
15
none
```

Todos trabajaron justo la media, así que nadie la supera.

### Ejemplo 3

Entrada:

```
5
ivo 7
jun 10
kai 0
lea 8
mo 24
```

Salida:

```
49
jun 10
mo 24
```

La media es 9,8: no tiene por qué ser un número entero.

### Ejemplo 4

Entrada:

```
1
hal 0
```

Salida:

```
0
none
```

### Restricciones

- 1 ≤ N ≤ 2·10⁴
- Cada nombre tiene de 1 a 12 letras minúsculas (`a`-`z`); los nombres son distintos.
- 0 ≤ horas ≤ 24
