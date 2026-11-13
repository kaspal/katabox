Un cine abre una segunda puerta y las dos colas de espera deben juntarse en una, por número de entrada. Cada cola, `first` y `second`, es una lista enlazada de números de entrada en orden no decreciente.

Júntalas en una sola cola, también en orden no decreciente, y devuélvela. Puedes reutilizar los nodos existentes.

Las listas se escriben como sus valores en orden: `[1, 4, 9]` es la lista 1 → 4 → 9, y `[]` es la lista vacía.

### Ejemplo 1

```
first = [1, 4, 9], second = [2, 4, 5, 12]
Salida: [1, 2, 4, 4, 5, 9, 12]
```

### Ejemplo 2

```
first = [], second = [3, 3]
Salida: [3, 3]
```

### Ejemplo 3

```
first = [], second = []
Salida: []
```

### Restricciones

- 0 ≤ longitud de cada cola ≤ 5·10⁴
- 1 ≤ número de entrada ≤ 10⁶
- Cada cola está en orden no decreciente.
