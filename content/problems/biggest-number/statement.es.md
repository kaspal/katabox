En una caseta de feria te dan un puñado de cartas con números y las colocas **todas** una junto a otra en fila para formar un número grande; la caseta paga más cuanto mayor es el número.

Dados los números de las cartas, devuelve el mayor número que puedes formar, como cadena. No tiene ceros a la izquierda, salvo que un número formado solo por ceros se escribe `"0"`.

### Ejemplo 1

```
cards = [7, 76, 760, 8]
Salida: "8776760"
```

El 8, luego el 7, luego el 76 y luego el 760.

### Ejemplo 2

```
cards = [0, 0]
Salida: "0"
```

### Ejemplo 3

```
cards = [40, 4, 404]
Salida: "440440"
```

### Restricciones

- 1 ≤ cards.length ≤ 1000
- 0 ≤ cards[i] ≤ 10⁹
