Un espacio de coworking recibe una lista de reuniones `meetings`, cada una `[start, end]` en minutos. Una reunión ocupa su sala desde `start` hasta `end`, sin incluir `end`: una reunión que termina en el minuto 30 y otra que empieza en el minuto 30 pueden usar la misma sala. Dos reuniones cuyos horarios se solapan necesitan salas distintas.

Devuelve el menor número de salas que permite celebrar todas las reuniones.

### Ejemplo 1

```
meetings = [[0, 40], [5, 15], [20, 35]]
Salida: 2
```

La primera reunión se solapa con las otras dos, pero esas dos no se solapan entre sí.

### Ejemplo 2

```
meetings = [[10, 20], [20, 30], [30, 40]]
Salida: 1
```

### Ejemplo 3

```
meetings = [[1, 5], [2, 6], [3, 7], [6, 8]]
Salida: 3
```

### Restricciones

- 1 ≤ meetings.length ≤ 2·10⁴
- meetings[i] = [start, end] con 0 ≤ start < end ≤ 10⁹
