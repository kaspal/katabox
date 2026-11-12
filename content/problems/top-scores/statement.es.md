Una bolera imprime un "salón de la fama" con las `k` mejores puntuaciones de la temporada. `scores` recoge todas las partidas jugadas, en el orden en que se jugaron; la misma puntuación puede aparecer muchas veces.

Devuelve las `k` puntuaciones más altas, **de mayor a menor**. Las puntuaciones iguales cuentan cada una, así que una puntuación puede salir más de una vez en la respuesta.

### Ejemplo 1

```
scores = [132, 201, 87, 201, 156, 99], k = 3
Salida: [201, 201, 156]
```

### Ejemplo 2

```
scores = [40, 75, 63], k = 3
Salida: [75, 63, 40]
```

### Ejemplo 3

```
scores = [300, 12, 300, 12], k = 1
Salida: [300]
```

### Restricciones

- 1 ≤ k ≤ scores.length ≤ 10⁵
- 0 ≤ scores[i] ≤ 300
