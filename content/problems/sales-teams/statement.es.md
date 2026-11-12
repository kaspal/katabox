Una cadena de pastelerías tiene `n` empleados, numerados del `0` al `n − 1`. `boss[i]` es el jefe directo del empleado `i`, o `-1` para la dueña, que al final es jefa de todos. El mes pasado el empleado `i` vendió `sales[i]` pasteles.

El **equipo** de un empleado es él mismo más todos los que están por debajo: las personas que dirige directamente, las que dirigen esas personas, y así sucesivamente. Para cada empleado, devuelve las ventas totales de su equipo.

### Ejemplo 1

```
boss = [-1, 0, 0, 1, 1], sales = [5, 3, 4, 2, 1]
Salida: [15, 6, 4, 2, 1]
```

El empleado 1 dirige al 3 y al 4, así que su equipo vendió 3 + 2 + 1 = 6. El equipo de la dueña es toda la cadena: 5 + 6 + 4 = 15.

### Ejemplo 2

```
boss = [-1], sales = [7]
Salida: [7]
```

### Ejemplo 3

```
boss = [1, -1, 0], sales = [1, 2, 3]
Salida: [4, 6, 3]
```

La dueña es la empleada 1, que dirige al 0, que dirige al 2.

### Restricciones

- 1 ≤ n ≤ 10⁵, sales.length = boss.length = n
- Exactamente un `boss[i]` es `-1`; cada uno de los demás está entre `0` y `n − 1`
- Siguiendo `boss` desde cualquiera siempre se llega a la dueña (no hay bucles)
- 0 ≤ sales[i] ≤ 10⁴
