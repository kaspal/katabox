Esta vez, en lugar de contar salas, asigna a cada reunión un **número de sala**. Las salas
se numeran 1, 2, 3, … y el coworking las reparte así:

- Las reuniones se atienden por orden de inicio; las que empiezan a la vez, en su orden en
  la entrada.
- Cada reunión recibe el **menor número de sala libre** en su inicio. Una sala está libre
  si todas las reuniones que ya tiene han terminado para entonces: como antes, una sala
  que se libera en el minuto `t` puede acoger una reunión que empieza en el minuto `t`.

Devuelve `rooms`, donde `rooms[i]` es el número de sala asignado a `meetings[i]`.

### Ejemplo 1

```
meetings = [[0, 30], [5, 10], [15, 20], [10, 40]]
Salida: [1, 2, 3, 2]
```

[10, 40] empieza cuando termina [5, 10], así que reutiliza la sala 2; en el minuto 15 las
salas 1 y 2 están ocupadas, así que [15, 20] abre la sala 3.

### Ejemplo 2

```
meetings = [[8, 27], [54, 60], [48, 51], [16, 20], [31, 56]]
Salida: [1, 2, 2, 2, 1]
```

### Ejemplo 3

```
meetings = [[55, 60], [3, 6], [5, 17], [53, 59], [47, 60]]
Salida: [3, 1, 2, 2, 1]
```

### Restricciones

- 1 ≤ meetings.length ≤ 2·10⁴
- meetings[i] = [start, end] con 0 ≤ start < end ≤ 10⁹
