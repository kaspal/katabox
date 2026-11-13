Varias líneas de autobús salen de la estación central de una ciudad. Cada línea sale por primera vez en el minuto `r` y después cada `m` minutos: en los minutos `r`, `r + m`, `r + 2·m`, y así sucesivamente. El ayuntamiento quiere una foto de todos los autobuses saliendo en el mismo instante, y los planificadores han preparado varios horarios. Para cada uno, ¿cuál es el primer minuto en que salen juntas todas sus líneas?

Este es un **problema de programa**: lee los horarios de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `T`, el número de horarios.
- Cada horario empieza con una línea con `n`, su número de líneas de autobús, seguida de `n` líneas `r m`: la línea sale por primera vez en el minuto `r` y después cada `m` minutos.

## Formato de salida

Escribe `T` líneas, una por horario y en orden: el primer minuto `t ≥ 0` en que salen todas las líneas de ese horario, o `-1` si eso no ocurre nunca. Las respuestas pueden acercarse a 9·10¹⁵, así que usa enteros de 64 bits (aún caben exactos en un número de JavaScript).

### Ejemplo 1

Entrada:

```
2
2
2 3
1 4
3
0 4
2 6
3 5
```

Salida:

```
5
8
```

En el segundo horario las líneas salen cada 4, 6 y 5 minutos; el minuto 8 es el primero que comparten.

### Ejemplo 2

Entrada:

```
3
2
0 4
1 6
1
7 9
2
3 10
3 10
```

Salida:

```
-1
7
3
```

En el primer horario una línea sale siempre en minutos pares y la otra en minutos impares, así que nunca coinciden.

### Ejemplo 3

Entrada:

```
1
3
4 7
2 9
0 10
```

Salida:

```
200
```

### Restricciones

- 1 ≤ T ≤ 2000
- 1 ≤ n ≤ 6
- 1 ≤ m ≤ 10⁶
- 0 ≤ r < m
- En cada horario, el mínimo común múltiplo de todos sus `m` es como mucho 9·10¹⁵.
