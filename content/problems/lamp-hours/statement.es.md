Una sala de lectura tiene varias lámparas con temporizador. La lámpara `i` está encendida desde el segundo `lamps[i][0]` hasta justo antes del segundo `lamps[i][1]`: el intervalo `[start, end)`. Las lámparas pueden solaparse, tocarse o estar muy separadas.

¿Durante cuántos segundos en total está encendida **al menos una** lámpara?

### Ejemplo 1

```
lamps = [[1, 4], [2, 6], [8, 10]]
Salida: 7
```

Las dos primeras lámparas juntas iluminan `[1, 6)`, 5 segundos; la tercera suma 2 más.

### Ejemplo 2

```
lamps = [[5, 7], [7, 9]]
Salida: 4
```

Las lámparas se tocan en el segundo 7, así que la luz no se apaga entre 5 y 9.

### Ejemplo 3

```
lamps = [[0, 1000000000]]
Salida: 1000000000
```

### Restricciones

- 1 ≤ lamps.length ≤ 10⁵
- `lamps[i] = [start, end]` con 0 ≤ start < end ≤ 10⁹
