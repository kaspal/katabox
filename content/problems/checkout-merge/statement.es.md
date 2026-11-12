A la hora del cierre, un supermercado junta todas sus colas de caja en una sola. Cada cola de `lines` es una lista enlazada con los números de turno de los clientes, del principio de la cola al final, ya en orden no decreciente.

Junta todas las colas en una sola, también en orden no decreciente de número de turno, y devuélvela. Puede haber colas vacías; si no hay ningún cliente, devuelve una cola vacía.

Las listas se escriben como sus valores en orden: `[2, 5, 9]` es la lista 2 → 5 → 9, y `[]` es la lista vacía.

### Ejemplo 1

```
lines = [[2, 5, 9], [1, 5], [3]]
Salida: [1, 2, 3, 5, 5, 9]
```

### Ejemplo 2

```
lines = [[], [4]]
Salida: [4]
```

### Ejemplo 3

```
lines = []
Salida: []
```

### Restricciones

- 0 ≤ lines.length ≤ 10⁴
- 0 ≤ número total de clientes ≤ 10⁵
- 1 ≤ número de turno ≤ 10⁶
- Cada cola está en orden no decreciente.
