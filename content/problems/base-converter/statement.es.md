Una caja de acertijos tiene diales que muestran números en distintas bases. Los dígitos van de `0` a `9` y después de `A` a `Z` para los valores de 10 a 35, así que la base 36 es la mayor.

Dado `number` escrito en base `source`, devuelve el mismo número escrito en base `target`, con letras mayúsculas y sin ceros a la izquierda (el cero es `"0"`).

### Ejemplo 1

```
number = "255", source = 10, target = 16
Salida: "FF"
```

### Ejemplo 2

```
number = "1011", source = 2, target = 10
Salida: "11"
```

### Ejemplo 3

```
number = "0", source = 7, target = 2
Salida: "0"
```

### Ejemplo 4

```
number = "Z", source = 36, target = 3
Salida: "1022"
```

Z es 35 = 1·27 + 0·9 + 2·3 + 2.

### Restricciones

- 2 ≤ source, target ≤ 36
- `number` usa solo dígitos válidos en base `source` (letras mayúsculas), no tiene ceros a la izquierda salvo que sea `"0"`, y su valor es como mucho 10¹⁵.
