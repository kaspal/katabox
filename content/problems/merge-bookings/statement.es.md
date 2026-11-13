La hoja de reservas de una sala de reuniones enumera franjas horarias, `bookings[i] = [start, end]`, con los dos momentos incluidos. Para imprimir las horas ocupadas, hay que combinar las reservas que se solapan: dos franjas que comparten al menos un momento (también solo un extremo) pasan a ser una franja que cubre las dos.

Devuelve las franjas combinadas, ordenadas por inicio.

### Ejemplo 1

```
bookings = [[4, 7], [1, 2], [6, 9], [12, 14]]
Salida: [[1, 2], [4, 9], [12, 14]]
```

### Ejemplo 2

```
bookings = [[5, 5], [5, 8], [9, 10]]
Salida: [[5, 8], [9, 10]]
```

[5, 8] y [9, 10] no comparten ningún momento, así que quedan separadas.

### Ejemplo 3

```
bookings = [[3, 20], [4, 6]]
Salida: [[3, 20]]
```

### Restricciones

- 1 ≤ bookings.length ≤ 10⁴
- 0 ≤ start ≤ end ≤ 10⁶
