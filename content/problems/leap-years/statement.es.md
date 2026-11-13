Una imprenta de calendarios necesita saber cuántos años, de `start` a `end` (ambos incluidos), tienen un 29 de febrero. Un año es **bisiesto** cuando:

- es divisible entre 4, **y**
- no es divisible entre 100, **salvo que** también sea divisible entre 400.

Así, 2024 y 2000 son bisiestos, pero 1900 y 2023 no.

Devuelve el número de años bisiestos de `start` a `end`.

### Ejemplo 1

```
start = 1996, end = 2004
Salida: 3
```

1996, 2000 y 2004.

### Ejemplo 2

```
start = 1900, end = 1900
Salida: 0
```

1900 es divisible entre 100 pero no entre 400.

### Ejemplo 3

```
start = 2000, end = 2000
Salida: 1
```

### Restricciones

- 1 ≤ start ≤ end ≤ 9999
