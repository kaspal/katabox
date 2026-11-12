Un puesto de envolver regalos tiene rollos de cinta de longitudes `ribbons` (en centímetros). Para un pedido grande necesita al menos `pieces` trozos de cinta, **todos de la misma longitud entera**. Cada rollo se puede cortar en tantos trozos de esa longitud como quepan; los restos se tiran y los trozos no se pueden pegar.

Devuelve la mayor longitud (al menos 1) que da al menos `pieces` trozos, o `0` si ni con trozos de longitud 1 hay suficientes.

### Ejemplo 1

```
ribbons = [30, 47, 18], pieces = 6
Salida: 15
```

La longitud 15 da 2 + 3 + 1 = 6 trozos. La longitud 16 solo da 1 + 2 + 1 = 4.

### Ejemplo 2

```
ribbons = [5, 5], pieces = 11
Salida: 0
```

Ni con trozos de 1 cm salen más de 10.

### Ejemplo 3

```
ribbons = [100], pieces = 1
Salida: 100
```

### Restricciones

- 1 ≤ ribbons.length ≤ 10⁵
- 1 ≤ ribbons[i] ≤ 10⁷
- 1 ≤ pieces ≤ 10⁹
