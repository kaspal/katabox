Unos voluntarios repintan una valla muy larga, medida en centímetros desde su extremo izquierdo. Cada entrada `[start, end]` de `strokes` dice que alguien pintó el tramo desde `start` hasta `end`, sin incluir `end`. Las pinceladas se solapan libremente; un centímetro pintado dos veces cuenta una sola vez.

Devuelve la longitud total de valla pintada.

### Ejemplo 1

```
strokes = [[2, 6], [10, 12], [4, 8]]
Salida: 8
```

[2, 6] y [4, 8] juntas cubren de 2 a 8 (6 cm); [10, 12] suma 2 cm.

### Ejemplo 2

```
strokes = [[0, 5], [5, 9]]
Salida: 9
```

Las pinceladas se tocan en el 5 sin solaparse.

### Ejemplo 3

```
strokes = [[3, 30], [7, 9], [3, 4]]
Salida: 27
```

### Restricciones

- 1 ≤ strokes.length ≤ 10⁵
- strokes[i] = [start, end] con 0 ≤ start < end ≤ 10⁹
