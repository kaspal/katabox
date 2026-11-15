La entrenadora de un club de atletismo anota cada vuelta que completa cada corredor en una sesión de entrenamiento. Al terminar quiere dos números por corredor: el tiempo total en la pista y la vuelta más rápida.

Los tiempos se escriben como `m:ss`: los minutos sin cero a la izquierda y luego los segundos, siempre con dos cifras (`0:59`, `4:05`, `12:30`). Los totales usan el mismo formato y siguen en minutos aunque pasen de una hora: 119 minutos y 58 segundos es `119:58`.

Este es un **problema de programa**: lee la hoja de la entrenadora de la entrada estándar y escribe el total y la vuelta más rápida de cada corredor en la salida estándar.

## Formato de entrada

- La primera línea contiene `T`, el número de corredores.
- Después, `T` líneas, una por corredor: su `name` (nombre), luego `k`, el número de vueltas, y luego los `k` tiempos de vuelta, todo separado por un espacio.

## Formato de salida

Escribe `T` líneas, una por corredor y en el orden de la entrada: `name total fastest`, separados por un espacio, donde `total` es la suma de sus vueltas y `fastest` la vuelta más corta, ambos en el formato `m:ss`.

### Ejemplo 1

Entrada:

```
3
ana 3 4:05 3:58 4:12
bo 1 12:30
carmen 4 0:59 1:01 1:00 0:59
```

Salida:

```
ana 12:15 3:58
bo 12:30 12:30
carmen 3:59 0:59
```

Ana corrió 245 + 238 + 252 = 735 segundos, es decir, 12 minutos y 15 segundos; su vuelta más rápida fue `3:58`. Un corredor con una sola vuelta tiene el mismo total y la misma vuelta más rápida.

### Ejemplo 2

Entrada:

```
1
zoe 2 59:59 59:59
```

Salida:

```
zoe 119:58 59:59
```

El total pasa de una hora y se sigue escribiendo en minutos.

### Ejemplo 3

Entrada:

```
2
li 5 7:07 7:00 6:59 7:10 7:03
mo 2 0:01 0:09
```

Salida:

```
li 35:19 6:59
mo 0:10 0:01
```

### Restricciones

- 1 ≤ T ≤ 100
- Un `name` tiene de 1 a 12 letras minúsculas (`a`-`z`).
- 1 ≤ k ≤ 50
- Cada vuelta se escribe `m:ss`, sin cero a la izquierda en los minutos, y `0:01` ≤ vuelta ≤ `59:59`.
