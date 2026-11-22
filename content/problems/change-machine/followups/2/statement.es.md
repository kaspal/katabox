Esta vez la máquina de cambio está casi vacía: solo tiene las fichas de `tokens`, y
**cada ficha se puede pagar como mucho una vez**. Varias fichas pueden tener el mismo
valor; cada una es una ficha distinta.

Devuelve el menor número de fichas de la máquina que suman exactamente `amount`, o `-1` si
ningún grupo de ellas lo consigue. Pagar `0` no necesita fichas.

### Ejemplo 1

```
tokens = [5, 1, 5, 2, 10], amount = 11
Salida: 2
```

10 + 1.

### Ejemplo 2

```
tokens = [3, 2, 5, 2, 8], amount = 15
Salida: 3
```

8 + 5 + 2; ningún par de fichas suma 15.

### Ejemplo 3

```
tokens = [1, 2, 2, 6, 3], amount = 11
Salida: 3
```

### Restricciones

- 1 ≤ tokens.length ≤ 100
- 1 ≤ tokens[i] ≤ 10⁴ (los valores pueden repetirse)
- 0 ≤ amount ≤ 10⁴
