Esta vez las canciones pueden repetirse todo lo que quieran, pero el tramo tiene que ceñirse a una rotación corta: como mucho `k` canciones **distintas**. Encuentra la serie más larga de emisiones consecutivas que no usa más de `k` números de canción distintos.

Devuelve su longitud.

### Ejemplo 1

```
plays = [7, 3, 7, 7, 3, 9, 3], k = 2
Salida: 5
```

Las emisiones 7, 3, 7, 7, 3 (posiciones 0 a 4) solo usan las canciones 7 y 3.

### Ejemplo 2

```
plays = [2, 1, 3, 1, 4, 4, 4, 4], k = 2
Salida: 5
```

Las emisiones 1, 4, 4, 4, 4 (posiciones 3 a 7).

### Ejemplo 3

```
plays = [1, 1, 1, 3, 2, 3, 3, 2], k = 100000
Salida: 8
```

Con una `k` tan grande, todo el registro vale.

### Restricciones

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
- 1 ≤ k ≤ 10⁵
