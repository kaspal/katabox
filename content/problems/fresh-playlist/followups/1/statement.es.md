El registro de la emisora es el mismo: `plays` enumera cada canción emitida, por su
número, y un **tramo sin repeticiones** es una serie de emisiones seguidas en la que
ninguna canción suena dos veces.

Ahora los presentadores preguntan por partes del archivo. Cada petición `[a, b]` pide la
longitud del tramo sin repeticiones más largo que queda entero dentro de las emisiones `a`
a `b` (las dos incluidas). Devuelve la respuesta de cada petición, en el orden en que se
hicieron.

### Ejemplo 1

```
plays = [4, 7, 4, 2, 7, 9, 2], requests = [[0, 6], [0, 2], [3, 5], [2, 4]]
Salida: [4, 2, 3, 3]
```

El tramo sin repeticiones más largo de todo el registro es `4, 2, 7, 9` (emisiones 2 a 5).
Dentro de las emisiones 0 a 2 (`4, 7, 4`) lo mejor es 2; las emisiones 3 a 5 (`2, 7, 9`) no
repiten nada; las emisiones 2 a 4 (`4, 2, 7`) tampoco.

### Ejemplo 2

```
plays = [1, 1, 1], requests = [[0, 2], [1, 1]]
Salida: [1, 1]
```

### Restricciones

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
- 1 ≤ requests.length ≤ 10⁵
- `requests[j] = [a, b]` con 0 ≤ a ≤ b < plays.length
