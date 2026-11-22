Esta vez el validador solo mira los paréntesis: `text` contiene letras minúsculas (que se siguen ignorando), `(` y `)`. En lugar de decir si el texto está equilibrado, devuelve el **menor número de paréntesis que hay que insertar**, en cualquier lugar de `text`, para que queden bien anidados: cada `(` se cierra con un `)` posterior y cada `)` cierra un `(` anterior.

Puedes insertar `(` o `)` en cualquier posición, pero no puedes borrar nada. Un texto que ya está equilibrado necesita 0 inserciones.

### Ejemplo 1

```
text = "())(a("
Salida: 3
```

El segundo `)` necesita un `(` insertado antes, y cada uno de los dos últimos `(` necesita un `)` después.

### Ejemplo 2

```
text = "()())"
Salida: 1
```

### Ejemplo 3

```
text = "a)aa"
Salida: 1
```

Basta con un `(` antes del `)`.

### Restricciones

- 1 ≤ text.length ≤ 10⁵
- `text` solo contiene letras minúsculas del alfabeto inglés, `(` y `)`.
