Un almacén etiqueta sus cajas con letras minúsculas y dígitos, como `box2` o `shelf10b`. Ordenar las etiquetas letra a letra pone `box10` antes que `box2`, lo que confunde a todo el mundo, así que el almacén quiere el **orden natural**:

- Divide cada etiqueta en **trozos**: tramos máximos de letras y tramos máximos de dígitos (`shelf10b` → `shelf`, `10`, `b`).
- Compara dos etiquetas trozo a trozo, desde la izquierda, hasta que dos trozos difieran:
  - dos trozos de dígitos se comparan por su **valor numérico** (`2` < `10`; `007` vale lo mismo que `7`);
  - dos trozos de letras se comparan **alfabéticamente**;
  - un trozo de dígitos va antes que un trozo de letras.
- Si una etiqueta se queda sin trozos antes y todos los trozos hasta entonces eran iguales, va primero.
- Las etiquetas iguales según estas reglas (como `a01` y `a1`) conservan su orden original.

Devuelve las etiquetas en orden natural.

### Ejemplo 1

```
labels = ["box10", "box2", "box1", "apple", "box2a"]
Salida: ["apple", "box1", "box2", "box2a", "box10"]
```

### Ejemplo 2

```
labels = ["a01", "a1", "a001"]
Salida: ["a01", "a1", "a001"]
```

Las tres son iguales en orden natural, así que se quedan como venían.

### Ejemplo 3

```
labels = ["9", "10", "x", "1x"]
Salida: ["1x", "9", "10", "x"]
```

### Restricciones

- 1 ≤ labels.length ≤ 10⁴
- 1 ≤ labels[i].length ≤ 20, solo letras minúsculas del alfabeto inglés y dígitos
