Un validador de archivos de configuración lee `text`, una línea formada por letras minúsculas y los corchetes `(`, `)`, `[`, `]`, `{` y `}`. Las letras se ignoran.

Devuelve `true` si los corchetes están bien anidados: cada corchete que abre se cierra más adelante con uno del mismo tipo, y un par que se abre dentro de otro también se cierra dentro de él. En caso contrario, devuelve `false`.

### Ejemplo 1

```
text = "f(a[b]{c})"
Salida: true
```

### Ejemplo 2

```
text = "(x]"
Salida: false
```

El `(` se cierra con un `]`, que es de otro tipo.

### Ejemplo 3

```
text = "({)}"
Salida: false
```

La `{` se abre dentro del par `(` pero se cierra después de él.

### Restricciones

- 1 ≤ text.length ≤ 10⁵
- `text` solo contiene letras minúsculas del alfabeto inglés y `()[]{}`.
