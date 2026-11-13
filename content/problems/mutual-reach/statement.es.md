Una app de mensajería tiene `n` usuarios, numerados de 0 a n − 1. Un enlace `[a, b]` significa que `a` puede reenviar mensajes a `b` (no al revés). Los mensajes pueden reenviarse a lo largo de varios enlaces.

Dos usuarios **se alcanzan mutuamente** cuando un mensaje de cualquiera de los dos puede llegar al otro. Eso divide a los usuarios en grupos en los que todos alcanzan a todos los del mismo grupo (un usuario solo es un grupo de uno).

Devuelve los tamaños de todos los grupos, en orden creciente.

### Ejemplo 1

```
n = 5, links = [[0, 1], [1, 2], [2, 0], [2, 3], [3, 4]]
Salida: [1, 1, 3]
```

Los usuarios 0, 1 y 2 se alcanzan mutuamente; 3 y 4 están solos, porque nada vuelve desde ellos.

### Ejemplo 2

```
n = 4, links = [[0, 1], [1, 0], [2, 3], [3, 2]]
Salida: [2, 2]
```

### Ejemplo 3

```
n = 3, links = []
Salida: [1, 1, 1]
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- 0 ≤ a, b < n y a ≠ b
