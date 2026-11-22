Esta vez cada charla tiene una **puntuación**: cuánto quiere verla el visitante. La charla
`i` es `talks[i] = [start, end, score]`. El visitante sigue asistiendo solo a charlas
completas, de una en una, y una charla que termina en un momento deja tiempo para empezar
otra en ese mismo momento.

Devuelve la **mayor puntuación total** de un conjunto de charlas al que pueda asistir. El
número de charlas ya no importa, solo su puntuación total.

### Ejemplo 1

```
talks = [[1, 4, 5], [3, 6, 9], [4, 7, 5], [6, 9, 2]]
Salida: 11
```

3–6 y 6–9 dan 9 + 2 = 11, mejor que 1–4 y 4–7 (5 + 5 = 10).

### Ejemplo 2

```
talks = [[2, 7, 3], [4, 5, 16], [7, 11, 13], [3, 4, 16]]
Salida: 45
```

3–4, 4–5 y 7–11 dan 16 + 16 + 13 = 45.

### Ejemplo 3

```
talks = [[0, 1, 3], [5, 7, 10], [4, 9, 7], [9, 10, 19]]
Salida: 32
```

### Restricciones

- 1 ≤ talks.length ≤ 10⁵
- talks[i] = [start, end, score] con 0 ≤ start < end ≤ 10⁶ y 1 ≤ score ≤ 10⁴
