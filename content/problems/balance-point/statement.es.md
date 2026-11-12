Una fila de cajas descansa sobre una tabla larga. La caja `i` empuja la tabla con una fuerza `weights[i]`; una fuerza negativa significa que la caja lleva atado un globo de helio que tira de ella hacia arriba.

Quieres poner el punto de apoyo bajo una caja de modo que la fuerza total a su izquierda sea igual a la de su derecha (la caja del apoyo no cuenta para ningún lado; un lado vacío suma 0).

Devuelve el **menor** índice donde el apoyo equilibra la tabla, o `-1` si no hay ninguno.

### Ejemplo 1

```
weights = [4, -1, 6, 2, 1]
Salida: 2
```

A la izquierda del índice 2: 4 + (-1) = 3. A su derecha: 2 + 1 = 3.

### Ejemplo 2

```
weights = [5, 3, -3]
Salida: 0
```

A la izquierda del índice 0 no hay nada (suma 0), y a la derecha 3 + (-3) = 0.

### Ejemplo 3

```
weights = [2, 7]
Salida: -1
```

### Restricciones

- 1 ≤ weights.length ≤ 10⁵
- −10⁴ ≤ weights[i] ≤ 10⁴
