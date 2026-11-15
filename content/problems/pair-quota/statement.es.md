La organizadora de una gincana de acertijos lleva un saldo de puntos para cada equipo. Los saldos suben con los acertijos resueltos y bajan con las penalizaciones, así que pueden ser negativos. Antes de la ronda final, dos equipos distintos pueden fusionarse en uno, pero solo si su saldo conjunto es como mucho un cupo `K`, para que los equipos fusionados no empiecen con demasiada ventaja. La organizadora quiere saber cuántas parejas de equipos podrían fusionarse. Organiza varias gincanas y hace la misma pregunta para cada una.

Este es un **problema de programa**: lee los casos de prueba de la entrada estándar y escribe un recuento por caso en la salida estándar.

## Formato de entrada

- La primera línea contiene `T`, el número de casos de prueba (gincanas).
- Después, cada caso ocupa dos líneas:
  - una línea con `n K`: el número de equipos y el cupo;
  - una línea con los `n` saldos, separados por espacios.

## Formato de salida

Escribe `T` líneas, una por caso y en orden: el número de parejas de equipos distintos cuyos saldos suman como mucho `K`. Cada pareja cuenta una vez, sin importar el orden, y dos equipos con el mismo saldo siguen siendo equipos distintos.

### Ejemplo 1

Entrada:

```
2
4 10
3 8 5 2
3 0
-4 4 1
```

Salida:

```
4
2
```

En el primer caso caben las parejas 3 + 5, 3 + 2, 8 + 2 y 5 + 2; 3 + 8 y 8 + 5 pasan de 10. En el segundo, -4 + 4 = 0 y -4 + 1 caben, 4 + 1 no.

### Ejemplo 2

Entrada:

```
1
1 5
7
```

Salida:

```
0
```

Un solo equipo no tiene con quién fusionarse.

### Ejemplo 3

Entrada:

```
3
5 -2
-1 -1 -1 -1 -1
2 2000000000
1000000000 1000000000
3 -2000000000
-1000000000 -1000000000 -1000000000
```

Salida:

```
10
1
3
```

### Restricciones

- 1 ≤ T ≤ 10⁴
- 1 ≤ n, y la suma de `n` en todos los casos es como mucho 10⁵
- -2·10⁹ ≤ K ≤ 2·10⁹
- -10⁹ ≤ saldo ≤ 10⁹
