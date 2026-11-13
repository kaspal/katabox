Un congreso ofrece muchas charlas; la charla `i` empieza en `talks[i][0]` y termina en `talks[i][1]`. Un visitante solo asiste a charlas completas, de una en una. Una charla que termina en un momento deja tiempo para empezar otra en ese mismo momento.

Devuelve el mayor número de charlas a las que puede asistir.

### Ejemplo 1

```
talks = [[9, 11], [10, 12], [11, 13], [12, 15], [8, 9]]
Salida: 3
```

Por ejemplo 8–9, 9–11 y 11–13.

### Ejemplo 2

```
talks = [[1, 100], [2, 3], [3, 4], [4, 5]]
Salida: 3
```

La charla larga no compensa.

### Ejemplo 3

```
talks = [[5, 6]]
Salida: 1
```

### Restricciones

- 1 ≤ talks.length ≤ 10⁵
- 0 ≤ talks[i][0] < talks[i][1] ≤ 10⁶
