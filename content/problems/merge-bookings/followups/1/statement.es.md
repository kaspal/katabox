Esta vez hay **dos** salas de reuniones, cada una con su propia hoja de reservas: `roomA` y
`roomB`. Cada reserva es `[start, end]`, con los dos momentos incluidos, y las reservas de
una misma hoja pueden solaparse. Encuentra los momentos en que **las dos salas están
ocupadas a la vez**.

Devuelve esos momentos como franjas combinadas `[start, end]` (con los dos extremos
incluidos), ordenadas por inicio. Como antes, las franjas que comparten un momento son una
sola, y las que no comparten ningún momento quedan separadas, aunque una termine justo
antes de que empiece la siguiente. Devuelve una lista vacía si las salas nunca están
ocupadas en el mismo momento.

### Ejemplo 1

```
roomA = [[1, 4], [8, 12], [3, 6]], roomB = [[5, 9], [11, 11]]
Salida: [[5, 6], [8, 9], [11, 11]]
```

La sala A está ocupada en [1, 6] y [8, 12]; la sala B, en [5, 9] y [11, 11].

### Ejemplo 2

```
roomA = [[4, 8], [2, 4], [3, 6]], roomB = [[15, 20], [12, 13], [3, 6]]
Salida: [[3, 6]]
```

### Ejemplo 3

```
roomA = [[1, 1], [2, 4], [5, 10]], roomB = [[9, 11], [19, 20], [19, 19], [18, 20]]
Salida: [[9, 10]]
```

### Restricciones

- 1 ≤ roomA.length, roomB.length ≤ 10⁴
- cada reserva es [start, end] con 0 ≤ start ≤ end ≤ 10⁶
