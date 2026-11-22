Wen echa una mano en la tienda de barrio de su tío y escribió una función que suma las ventas del día a la hora de cerrar. Casi todas las noches la caja cuadra, pero de vez en cuando el total sale un céntimo corto y su tío tiene que contar el cajón dos veces.

Cada elemento de `prices` es un precio escrito como cadena con exactamente dos decimales, por ejemplo `"12.30"`. Devuelve el total de todos los precios en **céntimos**, como entero (`"12.30"` solo da `1230`).

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
prices = ["0.10", "0.20", "4.35"]
Salida: 465
```

10 + 20 + 435 = 465 céntimos.

### Ejemplo 2

```
prices = ["5.17","2.01"]
Salida: 718
```

### Ejemplo 3

```
prices = ["4.35","25.25"]
Salida: 2960
```

### Restricciones

- 1 ≤ prices.length ≤ 10⁴
- Cada precio tiene la forma `12.30`: una parte entera de 0 a 9999 sin ceros a la izquierda, un punto y exactamente dos dígitos (de `"0.00"` a `"9999.99"`)
