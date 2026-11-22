Esta vez a la bolera le importa la **forma reciente**. `scores` recoge las partidas en el
orden en que se jugaron. Para cada ventana de `w` partidas consecutivas, informa de la
`k`-ésima puntuación más alta dentro de esa ventana. Las puntuaciones iguales cuentan cada
una, como antes.

Devuelve un valor por ventana, desde la ventana que empieza en la partida 0 hasta la que
termina en la última partida: `scores.length − w + 1` valores en total.

### Ejemplo 1

```
scores = [120, 200, 90, 250, 180], w = 3, k = 2
Salida: [120, 200, 180]
```

Las ventanas son [120, 200, 90], [200, 90, 250] y [90, 250, 180]; sus 2.ª puntuaciones más
altas son 120, 200 y 180.

### Ejemplo 2

```
scores = [291, 32, 130, 60, 253, 230], w = 3, k = 1
Salida: [291, 130, 253, 253]
```

### Ejemplo 3

```
scores = [46, 43, 184, 86, 157, 128], w = 3, k = 1
Salida: [184, 184, 184, 157]
```

### Restricciones

- 1 ≤ k ≤ w ≤ scores.length ≤ 10⁵
- 0 ≤ scores[i] ≤ 300
