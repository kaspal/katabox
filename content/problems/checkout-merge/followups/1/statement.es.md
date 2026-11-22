Esta vez el supermercado quiere **un tique de cada número** en la cola final. Como antes,
cada cola de `lines` es una lista enlazada con los números de turno, del principio de la
cola al final, ya en orden no decreciente, y el mismo número puede aparecer en varias
colas o varias veces en una misma cola.

Junta todas las colas en una sola en orden **estrictamente creciente**, conservando cada
número de turno exactamente una vez, y devuélvela. Puede haber colas vacías; si no hay
ningún cliente, devuelve una cola vacía.

Las listas se escriben como sus valores en orden: `[2, 5, 9]` es la lista 2 → 5 → 9, y
`[]` es la lista vacía.

### Ejemplo 1

```
lines = [[1, 4, 4, 9], [2, 4, 9], [1, 3]]
Salida: [1, 2, 3, 4, 9]
```

El turno 4 aparece tres veces, y el 1 y el 9 dos veces cada uno; cada uno se conserva una
vez.

### Ejemplo 2

```
lines = [[2, 2], [1, 2, 7, 7, 8], [1]]
Salida: [1, 2, 7, 8]
```

### Ejemplo 3

```
lines = [[2, 5, 6], [4, 7], [1, 3, 9]]
Salida: [1, 2, 3, 4, 5, 6, 7, 9]
```

### Restricciones

- 0 ≤ lines.length ≤ 10⁴
- 0 ≤ número total de clientes ≤ 10⁵
- 1 ≤ número de turno ≤ 10⁶
- Cada cola está en orden no decreciente.
