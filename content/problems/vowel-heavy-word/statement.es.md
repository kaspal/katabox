Lena está creando un juego de palabras y necesita saber cuántas vocales tiene, como máximo, una palabra de una lista. Escribió la función en un momento, pero los números que da casi nunca coinciden con lo que ella cuenta a mano.

Dado un arreglo `words` de palabras en minúsculas, devuelve la mayor cantidad de vocales que aparece en una sola palabra. Las vocales son `a`, `e`, `i`, `o` y `u` (la `y` no cuenta).

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
words = ["queue", "sky", "audio", "tree"]
Salida: 4
```

"queue" y "audio" tienen 4 vocales cada una; "sky" no tiene ninguna.

### Ejemplo 2

```
words = ["bebuuuodbu", "o", "auedbia", "a"]
Salida: 6
```

### Ejemplo 3

```
words = ["bi", "eed", "acooiueaai", "iooccdda"]
Salida: 9
```

### Restricciones

- 1 ≤ words.length ≤ 10⁴
- Cada palabra tiene de 1 a 20 letras minúsculas del alfabeto inglés.
