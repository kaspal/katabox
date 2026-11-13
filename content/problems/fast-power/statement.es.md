El generador de códigos de un cerrajero multiplica un número inicial por sí mismo una y otra vez: el código es `base` elevado a `exponent`. La respuesta puede ser enorme, así que devuélvela **módulo 1 000 003** (un primo): el resto de dividirla entre 1 000 003. Cualquier número elevado a 0 es 1 (también 0⁰).

Devuelve `base^exponent mod 1 000 003`.

### Ejemplo 1

```
base = 3, exponent = 13
Salida: 594320
```

3¹³ = 1 594 323, y 1 594 323 − 1 000 003 = 594 320.

### Ejemplo 2

```
base = 10, exponent = 0
Salida: 1
```

### Ejemplo 3

```
base = 1000003, exponent = 5
Salida: 0
```

### Ejemplo 4

```
base = 7, exponent = 1000000000000000
Salida: 942035
```

Multiplicar 10¹⁵ veces llevaría muchísimo tiempo.

### Restricciones

- 0 ≤ base ≤ 10⁹
- 0 ≤ exponent ≤ 10¹⁵
