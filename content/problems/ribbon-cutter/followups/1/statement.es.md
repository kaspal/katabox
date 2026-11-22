El puesto de envolver regalos del problema base ahora recibe de golpe los pedidos de
todo un día. Tiene rollos de cinta de longitudes enteras `ribbons` (en centímetros), y
`orders[k]` es el número de trozos que necesita el cliente `k`, todos de la misma
longitud entera. Como antes, cada rollo se corta en tantos trozos de esa longitud como
quepan, los restos se tiran y los trozos no se pueden pegar.

Los pedidos son independientes: cada uno se planifica con todos los rollos. Devuelve un
array cuya entrada `k` es la mayor longitud (al menos 1) que da al menos `orders[k]`
trozos, o `0` si ni con trozos de longitud 1 hay suficientes.

### Ejemplo 1

```
ribbons = [30, 47, 18], orders = [6, 1, 95, 200]
Salida: [15, 47, 1, 0]
```

La longitud 15 da 2 + 3 + 1 = 6 trozos. La longitud 1 da 95 trozos, que no bastan para
200.

### Ejemplo 2

```
ribbons = [37, 5], orders = [32, 31, 14]
Salida: [1, 1, 2]
```

La longitud 2 da 18 + 2 = 20 trozos; la longitud 3 solo da 12 + 1 = 13.

### Ejemplo 3

```
ribbons = [6, 6], orders = [3, 11, 5]
Salida: [3, 1, 2]
```

### Restricciones

- 1 ≤ ribbons.length, orders.length ≤ 10⁵
- 1 ≤ ribbons[i] ≤ 10⁵
- 1 ≤ orders[k] ≤ 10¹⁰ (más de lo que cabe en un entero de 32 bits)
