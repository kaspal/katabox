Una pequeña empresa comparte una sala de reuniones. Para cada día, la hoja de reservas indica cuándo abre la sala, cuándo cierra y los tramos que se han reservado. Los tramos se anotaron según llegaban: no están en orden y algunos se solapan. Un equipo quiere el tramo libre más largo de cada día.

Este es un **problema de programa**: lee la hoja de reservas de la entrada estándar y escribe el tramo libre más largo de cada día en la salida estándar.

## Formato de entrada

La primera línea contiene `T`, el número de días. Después, para cada día:

- una línea `open close k`: la hora de apertura y la de cierre de la sala y el número de tramos reservados;
- después, `k` líneas, cada una `start end`: un tramo reservado.

Todas las horas se escriben `hh:mm` en formato de 24 horas (dos cifras cada parte). Un tramo `start end` ocupa la sala desde `start` hasta `end`: el minuto `start` está ocupado y el minuto `end` ya está libre. Así, un tramo que termina a las `10:00` y otro que empieza a las `10:00` no dejan hueco entre ellos.

## Formato de salida

Escribe `T` líneas, una por día y en orden. Cada línea es el tramo libre más largo entre `open` y `close`, escrito `hh:mm-hh:mm` (el primer minuto libre y la hora en que termina). Si hay varios tramos igual de largos, escribe el más temprano. Si la sala no tiene ningún minuto libre ese día, escribe `BOOKED`.

### Ejemplo 1

Entrada:

```
2
09:00 17:00 3
10:00 11:30
13:00 14:00
10:30 12:00
08:00 12:00 0
```

Salida:

```
14:00-17:00
08:00-12:00
```

El primer día, los tramos `10:00 11:30` y `10:30 12:00` se solapan, así que la sala está ocupada de 10:00 a 12:00 y de 13:00 a 14:00. Los tramos libres son 09:00-10:00, 12:00-13:00 y 14:00-17:00, y el último es el más largo. El segundo día no tiene reservas, así que todo el día está libre.

### Ejemplo 2

Entrada:

```
1
09:00 10:00 2
09:00 09:40
09:30 10:00
```

Salida:

```
BOOKED
```

### Ejemplo 3

Entrada:

```
1
07:15 09:45 2
07:45 08:15
08:45 09:15
```

Salida:

```
07:15-07:45
```

Los tres tramos libres duran 30 minutos; gana el más temprano.

### Restricciones

- 1 ≤ T ≤ 100
- Las horas son `hh:mm`, de `00:00` a `23:59`
- open < close
- 0 ≤ k ≤ 1000
- open ≤ start < end ≤ close
- Σk ≤ 2·10⁴ (todos los días juntos)
