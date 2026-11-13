Una planta embotelladora empaqueta las botellas en lotes de `k`. Cada lote completo sale de la cinta en orden inverso (la última botella empaquetada sale primero), mientras que un lote final con menos de `k` botellas se envía tal cual.

La cinta es una lista enlazada de números de botella, `line`. Devuelve la cinta después de invertir cada lote completo de `k` botellas consecutivas. Cambia los enlaces, no los números guardados en los nodos.

Las listas se escriben como sus valores en orden: `[1, 4, 9]` es la lista 1 → 4 → 9, y `[]` es la lista vacía.

### Ejemplo 1

```
line = [1, 2, 3, 4, 5, 6, 7, 8], k = 3
Salida: [3, 2, 1, 6, 5, 4, 7, 8]
```

Las dos últimas botellas no llenan un lote.

### Ejemplo 2

```
line = [4, 9, 2], k = 1
Salida: [4, 9, 2]
```

### Ejemplo 3

```
line = [4, 9, 2], k = 3
Salida: [2, 9, 4]
```

### Restricciones

- 0 ≤ número de botellas ≤ 10⁵
- 1 ≤ número de botella ≤ 10⁶
- 1 ≤ k ≤ 10⁵
