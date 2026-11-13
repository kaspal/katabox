Una ciudad de canales tiene `n` islas, numeradas de 0 a n − 1, unidas por pasarelas. La pasarela `[a, b]` une las islas `a` y `b` en los dos sentidos; no hay dos pasarelas entre la misma pareja de islas.

Una pasarela es **crítica** cuando cerrarla dejaría alguna pareja de islas que antes se alcanzaban sin poder alcanzarse ahora. El ayuntamiento quiere revisar esas primero.

Devuelve las pasarelas críticas, cada una escrita como `[a, b]` con `a < b`, ordenadas por `a` y después por `b`.

### Ejemplo 1

```
n = 6, bridges = [[0, 1], [1, 2], [2, 3], [3, 1], [3, 4], [4, 5]]
Salida: [[0, 1], [3, 4], [4, 5]]
```

El circuito 1–2–3 resiste el cierre de cualquiera de sus pasarelas.

### Ejemplo 2

```
n = 4, bridges = [[0, 1], [1, 2], [2, 3], [3, 0]]
Salida: []
```

### Ejemplo 3

```
n = 3, bridges = [[2, 1]]
Salida: [[1, 2]]
```

La isla 0 no tiene ninguna pasarela.

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ bridges.length ≤ 2·10⁵
- 0 ≤ a, b < n, a ≠ b, y ninguna pareja está unida dos veces.
