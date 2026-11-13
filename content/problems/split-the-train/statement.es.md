Un tren de mercancías, `train`, es una lista enlazada de números de vagón desde la locomotora hacia atrás. En el cruce hay que dividirlo en exactamente `k` partes consecutivas, una por vía:

- las partes conservan el orden de los vagones y entre todas tienen todos los vagones;
- sus longitudes difieren como mucho en uno, y ninguna parte es más corta que otra posterior.

Algunas partes pueden quedar vacías cuando hay menos vagones que vías. Devuelve las `k` partes en orden.

Las listas se escriben como sus valores en orden: `[1, 4, 9]` es la lista 1 → 4 → 9, y `[]` es la lista vacía.

### Ejemplo 1

```
train = [1, 2, 3, 4, 5, 6, 7], k = 3
Salida: [[1, 2, 3], [4, 5], [6, 7]]
```

7 vagones en 3 vías: longitudes 3, 2, 2.

### Ejemplo 2

```
train = [8, 9], k = 4
Salida: [[8], [9], [], []]
```

### Ejemplo 3

```
train = [], k = 2
Salida: [[], []]
```

### Restricciones

- 0 ≤ número de vagones ≤ 10⁴
- 1 ≤ número de vagón ≤ 10⁶
- 1 ≤ k ≤ 100
