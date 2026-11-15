Una música ha recibido `n` ofertas de bolos (actuaciones) para la temporada. Cada oferta tiene una hora de inicio, una hora de fin y una paga. Solo puede tocar en un bolo a la vez, así que los que acepte no pueden solaparse; eso sí, un bolo puede empezar justo en el momento en que termina otro, porque la sala de al lado está a dos pasos.

Quiere la mayor paga total. Entre todos los planes que la alcanzan, prefiere el que tenga menos bolos, para cuidar la voz.

Este es un **problema de programa**: lee las ofertas de la entrada estándar y escribe la paga total y el número de bolos del mejor plan en la salida estándar.

## Formato de entrada

- La primera línea contiene `n`, el número de ofertas.
- Después, `n` líneas, cada una `start end pay`: la oferta va desde el instante `start` hasta el instante `end` y paga `pay`. Las ofertas no vienen en ningún orden concreto.

Dos ofertas se solapan cuando una empieza antes de que termine la otra. Una oferta que empieza exactamente en el `end` de otra no se solapa con ella.

## Formato de salida

Escribe una línea con dos números separados por un espacio: la mayor paga total y, después, el menor número de bolos con el que se alcanza esa paga.

### Ejemplo 1

Entrada:

```
4
1 4 50
4 6 30
1 6 80
6 9 20
```

Salida:

```
100 2
```

Dos planes pagan 100: `50 + 30 + 20`, con tres bolos seguidos, y `80 + 20`, con dos. El segundo tiene menos bolos.

### Ejemplo 2

Entrada:

```
1
0 1000000000 1000000000
```

Salida:

```
1000000000 1
```

### Ejemplo 3

Entrada:

```
3
0 10 5
2 3 5
5 8 5
```

Salida:

```
10 2
```

El bolo largo se solapa con los dos cortos; los dos cortos caben juntos y pagan 10.

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ start < end ≤ 10⁹
- 1 ≤ pay ≤ 10⁹
