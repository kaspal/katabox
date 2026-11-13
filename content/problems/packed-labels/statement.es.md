Un taller de rotulación imprime letras en cinta de señalización para suelos de aeropuertos y almacenes, y algunas cintas miden kilómetros. La impresora nunca podría guardar una cinta así letra por letra, de modo que guarda cada etiqueta **empaquetada**: una secuencia de tramos, cada uno escrito como una cantidad seguida de una letra minúscula. Por ejemplo, `3a2b10c` representa `aaabbcccccccccc`. La cantidad de un tramo siempre se escribe, aunque sea 1.

Antes de cada trabajo, el operario revisa letras sueltas: «¿qué letra hay en la posición `k` de la etiqueta `i`?».

Este es un **problema de programa**: lee las etiquetas y las revisiones de la entrada estándar y escribe las letras en la salida estándar.

## Formato de entrada

- La primera línea contiene `L`, el número de etiquetas.
- Cada una de las `L` líneas siguientes contiene una etiqueta empaquetada. Las etiquetas se numeran desde 1 en este orden.
- La línea siguiente contiene `Q`, el número de revisiones.
- Cada una de las `Q` líneas siguientes contiene dos números `i k`: la etiqueta `i` y la posición `k`, contada desde 1 en la etiqueta desempaquetada.

## Formato de salida

Escribe `Q` líneas, una por revisión y en orden: la letra que ocupa la posición `k` de la etiqueta `i` desempaquetada.

### Ejemplo 1

Entrada:

```
2
3a2b10c
1z
4
1 1
1 4
1 6
2 1
```

Salida:

```
a
b
c
z
```

La etiqueta 1 se desempaqueta como `aaabbcccccccccc`: las posiciones 1 a 3 son `a`, la 4 y la 5 son `b` y de la 6 a la 15 son `c`.

### Ejemplo 2

Entrada:

```
1
1000000000x5y999999999z
3
1 1000000000
1 1000000001
1 2000000004
```

Salida:

```
x
y
z
```

Esta etiqueta mide 2 000 000 004 letras: demasiado para desempaquetarla.

### Ejemplo 3

Entrada:

```
3
1a1b1c1d
2k1k3m
7q
3
1 3
2 4
3 7
```

Salida:

```
c
m
q
```

Dos tramos seguidos pueden usar la misma letra: `2k1k3m` se desempaqueta como `kkkmmm`.

### Restricciones

- 1 ≤ L ≤ 1000
- Cada etiqueta tiene al menos un tramo, y entre todas las etiquetas hay como mucho 60 000 tramos.
- 1 ≤ cantidad ≤ 10⁹, escrita sin ceros a la izquierda; las letras son minúsculas de la `a` a la `z`.
- 1 ≤ Q ≤ 20 000
- 1 ≤ i ≤ L, y 1 ≤ k ≤ la longitud desempaquetada de la etiqueta `i`.
