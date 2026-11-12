Un lector electrónico diminuto muestra `width` caracteres por línea. Maqueta un texto, dado como una lista de palabras `words`, así:

- Las palabras van a la línea actual, separadas por **un espacio**, mientras la línea tenga como mucho `width` caracteres.
- Cuando la siguiente palabra no cabe, la línea actual termina y la palabra empieza una línea nueva.
- Una palabra más larga que `width` nunca se parte: ocupa una línea para ella sola (el único caso en que una línea pasa de `width`).

Devuelve las líneas, en orden, sin espacios al final.

### Ejemplo 1

```
words = ["the", "quick", "brown", "fox", "jumps"], width = 10
Salida: ["the quick", "brown fox", "jumps"]
```

"the quick" tiene 9 caracteres; añadir " brown" daría 15.

### Ejemplo 2

```
words = ["encyclopedia", "is", "big"], width = 5
Salida: ["encyclopedia", "is", "big"]
```

"is big" necesitaría 6 caracteres.

### Ejemplo 3

```
words = ["a", "b", "c"], width = 5
Salida: ["a b c"]
```

### Restricciones

- 1 ≤ words.length ≤ 10⁴
- 1 ≤ words[i].length ≤ 20, letras minúsculas del alfabeto inglés
- 1 ≤ width ≤ 100
