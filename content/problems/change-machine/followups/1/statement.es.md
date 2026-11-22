La máquina de cambio de los recreativos sigue teniendo fichas ilimitadas de cada valor de
`coins`. Esta vez, en lugar del menor número de fichas, averigua **cuántas combinaciones
distintas** de fichas suman exactamente `amount`.

El orden no importa: una combinación es solo cuántas fichas de cada valor tomas, así que
`1 + 2` y `2 + 1` son la misma combinación. Pagar `0` tiene exactamente una combinación
(ninguna ficha). El número crece deprisa: devuélvelo módulo 1 000 000 007.

### Ejemplo 1

```
coins = [1, 2, 5], amount = 5
Salida: 4
```

5, 2 + 2 + 1, 2 + 1 + 1 + 1 y 1 + 1 + 1 + 1 + 1.

### Ejemplo 2

```
coins = [2, 5, 1], amount = 9
Salida: 8
```

### Ejemplo 3

```
coins = [7, 1, 6], amount = 6
Salida: 2
```

Una ficha de 6, o seis fichas de 1.

### Restricciones

- 1 ≤ coins.length ≤ 30
- 1 ≤ coins[i] ≤ 10⁴, todos distintos
- 0 ≤ amount ≤ 2·10⁴
