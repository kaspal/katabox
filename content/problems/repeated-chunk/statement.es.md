Un corrector sospecha que un largo manuscrito `text` contiene pasajes pegados dos veces. Encuentra la longitud del trozo más largo de `text` (una serie contigua de letras) que aparece **al menos dos veces**, en posiciones de inicio distintas. Las dos apariciones pueden solaparse.

Devuelve esa longitud, o `0` si ninguna secuencia de letras aparece dos veces.

### Ejemplo 1

```
text = "mississippi"
Salida: 4
```

"issi" empieza en las posiciones 1 y 4.

### Ejemplo 2

```
text = "abcd"
Salida: 0
```

### Ejemplo 3

```
text = "aaaa"
Salida: 3
```

"aaa" empieza en las posiciones 0 y 1.

### Restricciones

- 1 ≤ text.length ≤ 10⁵
- `text` tiene solo letras minúsculas del alfabeto inglés.
