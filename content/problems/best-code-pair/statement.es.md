Una fábrica de cerraduras da a cada llave un código numérico. Dos llaves forman una buena pareja para un sistema maestro cuando sus códigos difieren en el mayor número posible de bits altos: el **XOR** de los dos códigos (o exclusivo bit a bit) debe ser lo más grande posible.

Dados los códigos, `codes`, devuelve el mayor valor de `codes[i] XOR codes[j]` entre dos llaves distintas `i ≠ j`.

### Ejemplo 1

```
codes = [9, 14, 3, 20, 5]
Salida: 29
```

9 XOR 20 = 01001 XOR 10100 = 11101 = 29.

### Ejemplo 2

```
codes = [0, 0]
Salida: 0
```

### Ejemplo 3

```
codes = [7, 8]
Salida: 15
```

### Restricciones

- 2 ≤ codes.length ≤ 10⁵
- 0 ≤ codes[i] < 2²⁰ (es decir, menor que 1 048 576)
