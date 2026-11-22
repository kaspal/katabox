Amara organiza los turnos del mostrador de ayuda y quiere saber cuántas horas está atendido a la semana. Su script une los turnos que se solapan y suma el resultado, y casi todas las semanas acierta, pero a veces informa menos horas de las que el cuadrante muestra claramente.

Cada turno `[start, end]` de `shifts` cubre el tiempo de `start` a `end`, una duración de `end - start`. Los turnos pueden solaparse o tocarse. Devuelve la duración total del tiempo cubierto por al menos un turno.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
shifts = [[5, 6], [1, 4], [3, 9], [12, 15]]
Salida: 11
```

Los turnos cubren de 1 a 9 y de 12 a 15: 8 + 3 = 11.

### Ejemplo 2

```
shifts = [[18,20],[8,10],[15,20]]
Salida: 7
```

### Ejemplo 3

```
shifts = [[2,4],[11,14],[9,14]]
Salida: 7
```

### Restricciones

- 1 ≤ shifts.length ≤ 10⁴
- shifts[i] = [start, end] con 0 ≤ start < end ≤ 10⁶
