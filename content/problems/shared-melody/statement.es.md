Una app de música compara dos melodías, escritas como listas de números de nota, `first` y `second`. Una **melodía compartida** es una secuencia de notas que aparece en las dos melodías en el mismo orden, aunque no necesariamente seguidas (puede haber otras notas en medio).

Devuelve la longitud de la melodía compartida más larga.

### Ejemplo 1

```
first = [60, 62, 64, 65, 67], second = [62, 65, 60, 67]
Salida: 3
```

62, 65, 67 aparece en las dos, en ese orden.

### Ejemplo 2

```
first = [1, 2, 3], second = [4, 5]
Salida: 0
```

### Ejemplo 3

```
first = [7, 7, 7], second = [7, 7]
Salida: 2
```

### Restricciones

- 1 ≤ first.length, second.length ≤ 1500
- 0 ≤ número de nota ≤ 127
