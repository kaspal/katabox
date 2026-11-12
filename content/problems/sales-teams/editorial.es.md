## Enfoque: orden de arriba abajo y sumas de abajo arriba

El total del equipo del empleado `e` es `sales[e]` más los totales de los equipos de todos los que `e` dirige directamente. Eso invita a calcular los totales **de abajo arriba**, pero la numeración no dice quién está por encima de quién.

1. Construye las listas de hijos a partir de `boss` y localiza a la dueña.
2. Recorre desde la dueña nivel a nivel (en anchura). El orden resultante pone a cada jefe **antes** que a todo su equipo.
3. Empieza con `total = sales` y recorre el orden **al revés**: suma `total[e]` a `total[boss[e]]`. Cuando se llega a `e`, todos los que están por debajo de `e` van después en el orden y ya le han sumado su total, así que `total[e]` es definitivo cuando se pasa hacia arriba.

```
orden = BFS desde la dueña
total = copia de sales
para e en orden al revés:
    si boss[e] >= 0: total[boss[e]] += total[e]
```

### Paso a paso

`boss = [-1, 0, 0, 1, 1]`: el orden es 0, 1, 2, 3, 4. Al revés: el 4 suma 1 al empleado 1 (queda 4), el 3 suma 2 (queda 6), el 2 suma 4 a la dueña (9) y el 1 suma 6 (15). Totales: 15, 6, 4, 2, 1.

**Complejidad:** O(n) de tiempo y memoria. Sumar las ventas de cada empleado a todos sus jefes cuesta O(n · profundidad), hasta unas n²/2 en una sola cadena de mando.
