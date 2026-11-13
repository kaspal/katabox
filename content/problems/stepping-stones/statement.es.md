Un camino de piedras cruza un río: la piedra 0 está en una orilla y la piedra n − 1 en la otra. Desde la piedra `i`, un excursionista puede saltar hacia delante a cualquier piedra que esté como mucho `jumps[i]` piedras más allá (un valor 0 significa que no puede moverse desde ahí).

Empezando en la piedra 0, devuelve si el excursionista puede llegar a la última piedra.

### Ejemplo 1

```
jumps = [2, 1, 0, 3, 1]
Salida: false
```

Las piedras 1 y 2 son alcanzables, pero ninguna permite pasar de la piedra 2, donde el excursionista no puede moverse.

### Ejemplo 2

```
jumps = [2, 0, 3, 0, 0, 1]
Salida: true
```

0 → 2 → 5.

### Ejemplo 3

```
jumps = [0]
Salida: true
```

### Restricciones

- 1 ≤ jumps.length ≤ 10⁵
- 0 ≤ jumps[i] ≤ 10⁵
