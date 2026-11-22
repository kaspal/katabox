Las colas de las cajas son las mismas: cada cola de `lines` es una lista enlazada con los
números de turno, del principio de la cola al final, en orden no decreciente. Esta vez
cada cola tiene al menos un cliente.

Antes de cerrar, el encargado quiere una **muestra justa** para una encuesta: un rango de
números de turno `[lo, hi]` que contenga al menos el turno de un cliente de **cada** cola.
Devuelve el rango más estrecho como `[lo, hi]`: el menor `hi − lo` y, entre los rangos de
ese ancho, el de menor `lo`.

Las listas se escriben como sus valores en orden: `[2, 5, 9]` es la lista 2 → 5 → 9.

### Ejemplo 1

```
lines = [[3, 8, 20], [5, 9, 14], [1, 10, 25]]
Salida: [8, 10]
```

Los turnos 8, 9 y 10 vienen de las tres colas; ningún rango más estrecho llega a las tres.

### Ejemplo 2

```
lines = [[2, 2, 7], [7]]
Salida: [7, 7]
```

### Ejemplo 3

```
lines = [[6, 9]]
Salida: [6, 6]
```

### Restricciones

- 1 ≤ lines.length ≤ 10⁴
- 1 ≤ número total de clientes ≤ 10⁵, y cada cola tiene al menos uno
- 1 ≤ número de turno ≤ 10⁶
- Cada cola está en orden no decreciente.
