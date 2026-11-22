Esta vez la bolera quiere una clasificación **en directo**. `scores` recoge las partidas en
el orden en que se jugaron. Después de cada partida, informa de la `k`-ésima puntuación
más alta entre todas las partidas jugadas hasta ese momento, o `-1` mientras se hayan
jugado menos de `k` partidas.

Las puntuaciones iguales cuentan cada una, como antes: tras las partidas `[200, 200, 90]`
la 2.ª puntuación más alta es 200. Devuelve un valor por partida, en orden.

### Ejemplo 1

```
scores = [120, 200, 90, 250, 180], k = 2
Salida: [-1, 120, 120, 200, 200]
```

Tras la última partida, las puntuaciones de mayor a menor son 250, 200, 180, 120, 90, así
que la 2.ª más alta es 200.

### Ejemplo 2

```
scores = [68, 291, 32, 130, 60, 253], k = 3
Salida: [-1, -1, 32, 68, 68, 130]
```

### Ejemplo 3

```
scores = [28, 46, 43, 184, 86, 157], k = 3
Salida: [-1, -1, 28, 43, 46, 86]
```

### Restricciones

- 1 ≤ k ≤ scores.length ≤ 10⁵
- 0 ≤ scores[i] ≤ 300
