Un colegio numera sus taquillas 1, 2, 3, … a lo largo de un pasillo muy largo. Para una fiesta, cada taquilla cuyo número es **primo** se pinta de azul. (Un primo tiene exactamente dos divisores, 1 y él mismo, así que la taquilla 1 se queda gris).

Los profesores hacen muchas preguntas del mismo tipo: "¿cuántas taquillas azules hay desde la 1 hasta la `L`?". `limits` recoge el `L` de cada pregunta. Devuelve las respuestas en el mismo orden.

### Ejemplo 1

```
limits = [10, 2, 30]
Salida: [4, 1, 10]
```

Hasta la 10, las azules son la 2, la 3, la 5 y la 7.

### Ejemplo 2

```
limits = [1]
Salida: [0]
```

### Ejemplo 3

```
limits = [100, 100, 97]
Salida: [25, 25, 25]
```

### Restricciones

- 1 ≤ limits.length ≤ 2·10⁴
- 1 ≤ limits[i] ≤ 10⁶
