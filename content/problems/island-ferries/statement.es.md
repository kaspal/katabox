Un archipiélago tiene `n` islas, numeradas de `0` a `n − 1`. Cada entrada `[a, b]` de `links` es una línea de ferri que navega en los dos sentidos entre las islas `a` y `b`. Dos islas pertenecen al mismo **grupo** cuando puedes ir de una a otra en ferri, quizá cambiando de barco en otras islas.

Devuelve el número de grupos.

### Ejemplo 1

```
n = 6, links = [[0, 3], [4, 5], [3, 1]]
Salida: 3
```

Los grupos son {0, 1, 3}, {2} y {4, 5}.

### Ejemplo 2

```
n = 3, links = []
Salida: 3
```

Sin ferris, cada isla es su propio grupo.

### Ejemplo 3

```
n = 4, links = [[2, 1], [1, 2], [0, 3], [3, 2]]
Salida: 1
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 10⁵
- links[i] = [a, b] con 0 ≤ a, b < n y a ≠ b (la misma pareja puede aparecer más de una vez)
