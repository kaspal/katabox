Un pequeño cine muestra su plano de butacas como una lista de filas `rows`, todas de la misma longitud: `#` es una butaca vendida y `.` una libre.

Una butaca libre es **tranquila** cuando ninguna de las de alrededor está vendida: las de su izquierda y su derecha, las tres de la fila de delante y las tres de la fila de detrás. Las butacas del borde del plano simplemente tienen menos vecinas.

Devuelve cuántas butacas tranquilas hay.

### Ejemplo 1

```
rows = ["#...", "....", "..#."]
Salida: 3
```

Las tranquilas son las dos últimas de la primera fila y la primera de la última. Todas las demás libres tocan una vendida, de lado o en diagonal.

### Ejemplo 2

```
rows = ["...", "...", "..."]
Salida: 9
```

### Ejemplo 3

```
rows = ["#.#"]
Salida: 0
```

La butaca del medio está entre dos vendidas.

### Restricciones

- 1 ≤ rows.length ≤ 400
- 1 ≤ rows[i].length ≤ 400, todas las filas de la misma longitud
- Cada carácter es `#` o `.`
