El registro de la emisora es el mismo: `plays` enumera cada canción emitida, por su
número, y un **tramo sin repeticiones** es una serie de emisiones seguidas en la que
ninguna canción suena dos veces.

Para su pódcast la emisora corta partes del archivo en episodios. Cada petición `[a, b]`
toma las emisiones `a` a `b` (las dos incluidas) y las divide en **bloques** seguidos, cada
uno de los cuales debe ser un tramo sin repeticiones. Devuelve, para cada petición, el menor
número de bloques que basta, en el orden en que se hicieron.

### Ejemplo 1

```
plays = [4, 7, 4, 2, 7, 9, 2], requests = [[0, 6], [1, 4], [3, 5]]
Salida: [3, 2, 1]
```

Las emisiones 0 a 6 se dividen como `4, 7 | 4, 2, 7, 9 | 2`. Las emisiones 1 a 4, como
`7, 4, 2 | 7`. Las emisiones 3 a 5 (`2, 7, 9`) ya no repiten nada.

### Ejemplo 2

```
plays = [5, 5, 5, 5], requests = [[0, 3], [2, 3]]
Salida: [4, 2]
```

### Restricciones

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
- 1 ≤ requests.length ≤ 10⁵
- `requests[j] = [a, b]` con 0 ≤ a ≤ b < plays.length
