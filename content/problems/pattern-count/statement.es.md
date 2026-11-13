Un tejedor revisa una larga tira de tela estampada, escrita como la cadena `text`, buscando un motivo, la cadena `pattern`. Las apariciones pueden solaparse.

Devuelve cuántas posiciones `i` hay en las que `pattern` aparece en `text` empezando en `i`.

### Ejemplo 1

```
text = "tatatat", pattern = "tat"
Salida: 3
```

En las posiciones 0, 2 y 4.

### Ejemplo 2

```
text = "hello", pattern = "xyz"
Salida: 0
```

### Ejemplo 3

```
text = "aaaa", pattern = "aa"
Salida: 3
```

### Restricciones

- 1 ≤ text.length ≤ 2·10⁵
- 1 ≤ pattern.length ≤ 10⁵
- Las dos cadenas tienen solo letras minúsculas del alfabeto inglés.
