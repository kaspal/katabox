Para llenar un avión por los dos extremos a la vez, un agente de embarque reordena la cola: el primer pasajero, después el último, después el segundo, después el penúltimo, y así hasta colocarlos a todos.

La cola es una lista enlazada de números de pasajero, `queue`, de delante hacia atrás. Devuélvela en orden de zigzag: L0, Ln−1, L1, Ln−2, L2, …

Las listas se escriben como sus valores en orden: `[1, 4, 9]` es la lista 1 → 4 → 9, y `[]` es la lista vacía.

### Ejemplo 1

```
queue = [3, 8, 1, 6, 9]
Salida: [3, 9, 8, 6, 1]
```

### Ejemplo 2

```
queue = [10, 20, 30, 40]
Salida: [10, 40, 20, 30]
```

### Ejemplo 3

```
queue = [7]
Salida: [7]
```

### Restricciones

- 0 ≤ número de pasajeros ≤ 10⁵
- 1 ≤ número de pasajero ≤ 10⁶
