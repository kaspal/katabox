La lavandería clasifica los calcetines por el número cosido en cada uno. Tras el gran lavado del lunes, la máquina devuelve una lista de números, `ids`. Cada número aparece exactamente **dos veces**, porque los calcetines van por pares, salvo un calcetín cuya pareja se ha perdido.

Devuelve el número del calcetín solitario.

### Ejemplo 1

```
ids = [14, 3, 9, 3, 14]
Salida: 9
```

### Ejemplo 2

```
ids = [72]
Salida: 72
```

Solo salió un calcetín, así que no tiene pareja.

### Ejemplo 3

```
ids = [5, 8, 8, 1, 6, 1, 5]
Salida: 6
```

### Restricciones

- 1 ≤ ids.length ≤ 10⁵ (siempre impar)
- 1 ≤ ids[i] ≤ 10⁹
- Cada valor aparece exactamente dos veces, salvo uno que aparece una sola vez.
