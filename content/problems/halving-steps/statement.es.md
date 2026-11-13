Un juego de acertijos muestra un número `n` y una regla por paso:

- si el número es **par**, se divide entre dos;
- si es **impar** (y no es 1), se multiplica por 3 y se le suma 1.

El juego termina cuando el número llega a 1. Devuelve cuántos pasos hacen falta para llegar a 1 empezando desde `n`.

### Ejemplo 1

```
n = 6
Salida: 8
```

6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1: ocho pasos.

### Ejemplo 2

```
n = 1
Salida: 0
```

### Ejemplo 3

```
n = 16
Salida: 4
```

### Restricciones

- 1 ≤ n ≤ 10⁶
- Todo n de este rango llega a 1, y los números del camino se mantienen por debajo de 10¹¹.
