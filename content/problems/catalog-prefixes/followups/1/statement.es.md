Esta vez el catálogo cambia mientras los lectores buscan. Empieza vacío, y cada `ops[i]`, junto con `words[i]`, es una de estas operaciones:

- `"add"`: una copia del título `words[i]` entra en el catálogo (las copias cuentan por separado, como antes);
- `"remove"`: una copia de `words[i]` sale del catálogo (siempre está);
- `"count"`: un lector escribió `words[i]`; indica cuántos títulos del catálogo empiezan por él **en ese momento**.

Devuelve las respuestas de las operaciones `"count"` en orden (una lista vacía si no hay ninguna). Un título empieza por sí mismo.

### Ejemplo 1

```
ops = ["add", "add", "count", "add", "remove", "count", "count"], words = ["moby", "moon", "mo", "mob", "moby", "mob", "z"]
Salida: [2, 1, 0]
```

Una vez retirado "moby", solo "mob" empieza por "mob".

### Ejemplo 2

```
ops = ["add", "remove", "add", "add", "count", "count"], words = ["b", "b", "aa", "ba", "a", "a"]
Salida: [1, 1]
```

### Ejemplo 3

```
ops = ["count", "add", "add", "add", "count", "count"], words = ["a", "b", "a", "b", "b", "a"]
Salida: [0, 2, 1]
```

La primera consulta ve el catálogo vacío.

### Restricciones

- 1 ≤ ops.length ≤ 3·10⁴
- words.length = ops.length
- Cada `ops[i]` es `"add"`, `"remove"` o `"count"`
- 1 ≤ words[i].length ≤ 20, solo letras minúsculas del alfabeto inglés
- `"remove"` solo nombra un título que está en el catálogo en ese momento
