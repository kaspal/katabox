Una grabadora de campo instalada en un bosque anota, cada segundo, qué fuente de sonido fue la más fuerte: un pájaro carpintero, un arroyo, un camión que pasa, cada una con su propio número de id. Un tramo de segundos consecutivos es una **mezcla K** cuando en él aparecen exactamente `K` fuentes distintas. La diseñadora de sonido quiere saber cuántos tramos de la grabación son mezclas K.

Este es un **problema de programa**: lee la grabación de la entrada estándar y escribe el recuento en la salida estándar.

## Formato de entrada

- La primera línea contiene `n K`: el número de segundos grabados y el número de fuentes distintas que debe tener una mezcla K.
- La segunda línea contiene los `n` ids de fuente, separados por espacios, uno por segundo y en orden.

## Formato de salida

Escribe una línea: el número de tramos (del segundo `i` al segundo `j`, con `i ≤ j`) que contienen exactamente `K` ids distintos.

### Ejemplo 1

Entrada:

```
6 2
4 4 7 4 9 9
```

Salida:

```
7
```

Las mezclas 2 son `4 4 7`, `4 4 7 4`, `4 7`, `4 7 4`, `7 4`, `4 9` y `4 9 9`. El tramo `7 4 9` no lo es: tiene tres fuentes.

### Ejemplo 2

Entrada:

```
3 3
1000000000 5 1000000000
```

Salida:

```
0
```

En toda la grabación solo aparecen dos fuentes distintas, así que ningún tramo tiene tres.

### Ejemplo 3

Entrada:

```
5 1
8 8 8 3 3
```

Salida:

```
9
```

### Restricciones

- 1 ≤ K ≤ n ≤ 10⁵
- 1 ≤ id ≤ 10⁹, escrito sin ceros a la izquierda
