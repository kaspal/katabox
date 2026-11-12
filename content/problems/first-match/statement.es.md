Un ayudante de crucigramas guarda una lista de palabras. Cuando quien resuelve escribe las primeras letras de una respuesta, el ayudante propone **la primera palabra en orden alfabético** de la lista que empieza por esas letras (una palabra empieza por sí misma). Si ninguna palabra empieza por ellas, no propone nada.

`words` es la lista de palabras (puede tener repeticiones) y `prefixes` recoge lo que se escribió. Devuelve la propuesta para cada prefijo, o `""` si no hay ninguna.

### Ejemplo 1

```
words = ["pear", "peach", "plum", "apple"], prefixes = ["pe", "p", "a", "b", "peac"]
Salida: ["peach", "peach", "apple", "", "peach"]
```

"peach" va antes que "pear" y "plum" en orden alfabético.

### Ejemplo 2

```
words = ["tree"], prefixes = ["tree", "trees", "t"]
Salida: ["tree", "", "tree"]
```

### Ejemplo 3

```
words = ["ab", "a", "abc"], prefixes = ["a", "ab"]
Salida: ["a", "ab"]
```

Una palabra más corta va antes que las palabras más largas que empiezan por ella.

### Restricciones

- 1 ≤ words.length ≤ 10⁴, 1 ≤ prefixes.length ≤ 2·10⁴
- 1 ≤ words[i].length, prefixes[j].length ≤ 20
- Solo letras minúsculas del alfabeto inglés
