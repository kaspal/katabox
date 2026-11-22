Amara lleva la taquilla de un teatro pequeño y escribió una función que dibuja el plano de asientos después de las reservas de la noche. En su primera prueba, con una sola fila, funcionó a la perfección. Luego un cliente reservó un asiento en una sala más grande y el plano mostró ocupados asientos que nadie había reservado.

La sala tiene `rows` filas y `cols` asientos por fila, y todos los asientos empiezan libres. Cada elemento de `bookings` es `[row, col]` y marca ese asiento como ocupado; un asiento puede reservarse más de una vez y simplemente sigue ocupado. Devuelve el plano como una lista de `rows` cadenas, una por fila empezando por la fila 0, cada una de longitud `cols`, donde `.` es un asiento libre y `X` uno ocupado.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
rows = 3, cols = 4, bookings = [[0, 1], [2, 3], [0, 1]]
Salida: [".X..", "....", "...X"]
```

El asiento `[0, 1]` se reserva dos veces; sigue estando simplemente ocupado.

### Ejemplo 2

```
rows = 2, cols = 3, bookings = [[0,1],[1,1]]
Salida: [".X.",".X."]
```

### Ejemplo 3

```
rows = 2, cols = 3, bookings = [[1,0]]
Salida: ["...","X.."]
```

### Restricciones

- 1 ≤ rows, cols ≤ 300
- 0 ≤ bookings.length ≤ 10⁴
- Cada reserva es `[row, col]` con 0 ≤ row < rows y 0 ≤ col < cols
