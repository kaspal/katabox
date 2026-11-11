Las etiquetas de los estantes de un almacén son demasiado estrechas para los códigos largos, así que los códigos se acortan. Divide `text` en **rachas**: grupos máximos de letras iguales consecutivas. Escribe cada racha como su letra seguida de la longitud de la racha **solo si esa longitud es mayor que 1**.

Devuelve la etiqueta acortada.

### Ejemplo 1

```
text = "aaabcc"
Output: "a3bc2"
```

Las rachas son `aaa`, `b` y `cc`.

### Ejemplo 2

```
text = "abab"
Output: "abab"
```

### Ejemplo 3

```
text = "zzzzzzzzzzzz"
Output: "z12"
```

### Restricciones

- 1 ≤ text.length ≤ 10⁵
- `text` solo contiene letras minúsculas del alfabeto inglés.
