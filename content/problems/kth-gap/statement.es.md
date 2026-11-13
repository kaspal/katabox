A lo largo de una carretera recta hay postes meteorológicos; `posts[i]` es el punto kilométrico del poste `i` (dos postes pueden compartir punto). Para cada par de postes, la distancia es lo que los separa. Ordena las distancias de los n · (n − 1) / 2 pares de menor a mayor.

Devuelve la distancia número `k` en ese orden (contando desde 1).

### Ejemplo 1

```
posts = [10, 2, 7, 3], k = 4
Salida: 5
```

Las seis distancias son 8, 3, 7, 5, 1 y 4; ordenadas, 1, 3, 4, 5, 7, 8.

### Ejemplo 2

```
posts = [5, 5, 9], k = 1
Salida: 0
```

### Ejemplo 3

```
posts = [0, 1000000], k = 1
Salida: 1000000
```

### Restricciones

- 2 ≤ posts.length ≤ 5·10⁴
- 0 ≤ posts[i] ≤ 10⁶
- 1 ≤ k ≤ n · (n − 1) / 2
