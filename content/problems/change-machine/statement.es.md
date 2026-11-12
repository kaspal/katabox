La máquina de cambio de unos recreativos paga en fichas. Tiene fichas ilimitadas de cada valor de `coins` y siempre paga una cantidad exacta `amount` con el **menor número de fichas** posible.

Devuelve el menor número de fichas que suman exactamente `amount`, o `-1` si ninguna combinación de los valores da esa cantidad. Pagar `0` no necesita fichas.

### Ejemplo 1

```
coins = [1, 6, 10], amount = 12
Salida: 2
```

Dos fichas de 6. Empezar por la ficha más grande (10 + 1 + 1) usaría tres.

### Ejemplo 2

```
coins = [4, 6], amount = 9
Salida: -1
```

Cualquier combinación de 4 y 6 es par.

### Ejemplo 3

```
coins = [5], amount = 0
Salida: 0
```

### Restricciones

- 1 ≤ coins.length ≤ 30
- 1 ≤ coins[i] ≤ 10⁴, todos distintos
- 0 ≤ amount ≤ 2·10⁴
