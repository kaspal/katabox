Un reloj binario muestra un número como una fila de luces, una por dígito binario: una luz está encendida para un 1. El relojero quiere una tabla de cuántas luces se encienden para cada número de 0 a `n`.

Devuelve una lista cuya entrada `i` es el número de bits a 1 en la forma binaria de `i`, para `i = 0, 1, …, n`.

### Ejemplo 1

```
n = 6
Salida: [0, 1, 1, 2, 1, 2, 2]
```

5 es 101 en binario (2 luces), 6 es 110 (2 luces).

### Ejemplo 2

```
n = 0
Salida: [0]
```

### Ejemplo 3

```
n = 9
Salida: [0, 1, 1, 2, 1, 2, 2, 3, 1, 2]
```

### Restricciones

- 0 ≤ n ≤ 10⁵
