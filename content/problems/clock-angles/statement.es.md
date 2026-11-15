Una relojera está probando una nueva esfera analógica: un reloj corriente de 12 horas con una aguja horaria y un minutero. Para una lista de horas del día, escritas en formato de 24 horas, quiere saber el ángulo entre las dos agujas, para comprobarlas contra un dibujo.

El minutero gira 6 grados cada minuto. La aguja horaria también avanza de forma continua: 30 grados cada hora, es decir, medio grado cada minuto, así que a las 09:45 está a tres cuartos del camino entre el 9 y el 10. El ángulo pedido es el menor de los dos que forman las agujas, entre 0 y 180 grados.

Este es un **problema de programa**: lee las horas de la entrada estándar y escribe los ángulos en la salida estándar.

## Formato de entrada

- La primera línea contiene `Q`, el número de horas.
- Después, `Q` líneas, cada una una hora `hh:mm` en formato de 24 horas, con dos cifras para las horas y dos para los minutos.

## Formato de salida

Escribe `Q` líneas, una por hora y en orden: el menor ángulo entre las agujas, en grados, con una cifra después del punto decimal.

### Ejemplo 1

Entrada:

```
4
03:00
09:45
12:00
06:30
```

Salida:

```
90.0
22.5
0.0
15.0
```

A las 09:45 el minutero apunta al 9, a 270 grados del 12. La aguja horaria está en 9 · 30 + 45 · 0.5 = 292.5 grados, así que las agujas están a 22.5 grados.

### Ejemplo 2

Entrada:

```
2
18:00
00:59
```

Salida:

```
180.0
35.5
```

A las 18:00 la esfera marca las 6, y las agujas apuntan en sentidos opuestos.

### Ejemplo 3

Entrada:

```
3
23:59
01:05
14:20
```

Salida:

```
5.5
2.5
50.0
```

### Restricciones

- 1 ≤ Q ≤ 1440
- 00 ≤ `hh` ≤ 23
- 00 ≤ `mm` ≤ 59
