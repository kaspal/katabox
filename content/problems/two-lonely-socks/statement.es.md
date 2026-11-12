Vuelve la lavandería, y esta vez **dos** calcetines perdieron su pareja. La máquina apunta el número cosido en cada calcetín que encontró, `ids`: cada número aparece exactamente dos veces, salvo dos números distintos que aparecen una vez cada uno.

Devuelve los dos números solitarios en **orden creciente**.

¿Puedes hacerlo con una pasada por la lista y una cantidad constante de memoria adicional, sin ordenar ni usar un mapa hash?

### Ejemplo 1

```
ids = [8, 5, 13, 5, 2, 8]
Salida: [2, 13]
```

### Ejemplo 2

```
ids = [41, 40]
Salida: [40, 41]
```

### Ejemplo 3

```
ids = [6, 1, 6, 9, 1, 3]
Salida: [3, 9]
```

### Restricciones

- 2 ≤ ids.length ≤ 10⁵ + 2
- 1 ≤ ids[i] ≤ 10⁹
- Cada número aparece exactamente dos veces, salvo dos números distintos que aparecen una vez
