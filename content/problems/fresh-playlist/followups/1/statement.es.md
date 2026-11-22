Esta vez la emisora permite un bis: una canción puede sonar **dos veces** en un tramo, pero no tres. Encuentra la serie más larga de emisiones consecutivas en la que ningún número de canción aparece más de dos veces.

Devuelve su longitud.

### Ejemplo 1

```
plays = [7, 3, 7, 7, 3, 9, 3]
Salida: 5
```

Por ejemplo 3, 7, 7, 3, 9 (posiciones 1 a 5); cualquier tramo más largo emite el 7 o el 3 tres veces.

### Ejemplo 2

```
plays = [2, 1, 3, 1, 4, 4, 4, 4]
Salida: 6
```

Las emisiones 2, 1, 3, 1, 4, 4 (posiciones 0 a 5).

### Ejemplo 3

```
plays = [1, 1, 1, 3, 2, 3, 3, 2]
Salida: 5
```

### Restricciones

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
