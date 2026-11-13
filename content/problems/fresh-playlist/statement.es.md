Una emisora de radio registra cada canción que emite, por número de canción, en `plays`. La dirección quiere anunciar su **tramo fresco** más largo: una serie de emisiones consecutivas en la que ninguna canción sonó dos veces.

Devuelve la longitud del tramo fresco más largo.

### Ejemplo 1

```
plays = [4, 8, 4, 2, 9, 8, 1]
Salida: 5
```

Las emisiones 4, 2, 9, 8, 1 (posiciones 2 a 6).

### Ejemplo 2

```
plays = [7, 7, 7]
Salida: 1
```

### Ejemplo 3

```
plays = [3, 1, 2]
Salida: 3
```

### Restricciones

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
