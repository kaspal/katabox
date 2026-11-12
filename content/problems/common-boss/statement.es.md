Una gran cooperativa tiene `n` socios, numerados del `0` al `n − 1`. `boss[i]` es el jefe directo del socio `i`, o `-1` para la presidenta, que está por encima de todos.

Cuando dos socios discrepan, la disputa va a su **jefe común más cercano**: la persona más baja de la jerarquía que es uno de los dos o está por encima de ambos (directamente o a través de otros jefes). Si un socio está por encima del otro, decide ese socio; un socio es su propio jefe común más cercano.

`pairs[j] = [a, b]` describe la disputa `j`. Devuelve, para cada disputa, quién la decide.

### Ejemplo 1

```
boss = [-1, 0, 0, 1, 1, 2], pairs = [[3, 4], [3, 5], [4, 1], [2, 2]]
Salida: [1, 0, 1, 2]
```

Los socios 3 y 4 dependen ambos del 1. El socio 3 está bajo el 1 y el 5 bajo el 2, así que solo la presidenta está por encima de ambos. El socio 1 está por encima del 4, así que decide el 1.

### Ejemplo 2

```
boss = [-1], pairs = [[0, 0]]
Salida: [0]
```

### Ejemplo 3

```
boss = [2, 0, -1, 1], pairs = [[3, 0], [1, 3]]
Salida: [0, 1]
```

La jerarquía es una línea: el 2 por encima del 0, el 0 del 1 y el 1 del 3.

### Restricciones

- 1 ≤ n ≤ 5·10⁴
- Exactamente un `boss[i]` es `-1`; cada uno de los demás está entre `0` y `n − 1`
- Siguiendo `boss` desde cualquiera siempre se llega a la presidenta (no hay bucles)
- 1 ≤ pairs.length ≤ 5·10⁴, `pairs[j] = [a, b]` con 0 ≤ a, b < n
