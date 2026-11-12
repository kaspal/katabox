Una furgoneta de reparto cruza una ciudad dividida en una cuadrícula de distritos. Entrar en el distrito `(r, c)` cuesta un peaje de `tolls[r][c]` monedas, y eso incluye el distrito de salida, arriba a la izquierda `(0, 0)`, y el de destino, abajo a la derecha. Desde un distrito la furgoneta puede moverse arriba, abajo, a la izquierda o a la derecha a uno vecino, y paga de nuevo cada vez que entra en un distrito.

El conductor tiene además **un cupón**: en un único distrito de su elección paga la mitad del peaje, redondeando hacia abajo. El cupón se puede usar como mucho una vez (también en la salida o en el destino).

Devuelve las monedas mínimas necesarias para ir del distrito de arriba a la izquierda al de abajo a la derecha.

### Ejemplo 1

```
tolls = [[1, 9, 1], [1, 9, 1], [1, 1, 1]]
Salida: 4
```

Bajando por la columna izquierda y siguiendo la fila de abajo: cinco distritos de peaje 1 cuestan 5, y el cupón hace que uno de ellos cueste 0.

### Ejemplo 2

```
tolls = [[7]]
Salida: 3
```

La salida es también el destino; con el cupón su peaje de 7 cuesta 3.

### Ejemplo 3

```
tolls = [[1, 100, 1], [50, 100, 50], [1, 100, 1]]
Salida: 103
```

Todas las rutas cruzan la columna central. Pasar por `(0, 1)` cuesta 1 + 100 + 1 + 50 + 1 = 153, y el cupón sobre el 100 ahorra 50.

### Restricciones

- 1 ≤ filas, columnas ≤ 150, todas las filas de `tolls` tienen la misma longitud
- 1 ≤ tolls[r][c] ≤ 10⁴
