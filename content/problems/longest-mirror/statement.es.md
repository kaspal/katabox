A un rotulista le gustan las palabras que se leen igual al revés, como `level` o `noon`. Dada una línea de texto en minúsculas `text`, encuentra el trozo **contiguo** más largo que se lee igual al revés.

Devuelve ese trozo. Si varios trozos tienen la mayor longitud, devuelve el que empieza antes.

### Ejemplo 1

```
text = "levels"
Salida: "level"
```

### Ejemplo 2

```
text = "noonmadam"
Salida: "madam"
```

### Ejemplo 3

```
text = "xy"
Salida: "x"
```

Cada letra suelta se lee igual al revés; la `x` va primero.

### Restricciones

- 1 ≤ text.length ≤ 3000
- `text` tiene solo letras minúsculas del alfabeto inglés.
