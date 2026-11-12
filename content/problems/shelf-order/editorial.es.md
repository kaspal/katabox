## Enfoque: ordenar por una clave de trozos

Convierte cada etiqueta en una **clave**: una lista con una entrada por trozo.

- Un trozo de dígitos se convierte en `(0, longitud sin ceros a la izquierda, dígitos sin ceros a la izquierda)`. Comparar esto compara valores numéricos sin pasar a enteros: un número más corto es menor, y con la misma longitud se comparan dígito a dígito. `007` y `7` dan la misma entrada.
- Un trozo de letras se convierte en `(1, letras)`, así que los dígitos van antes que las letras y las letras se comparan alfabéticamente.

Comparar dos claves entrada a entrada es exactamente el orden natural, incluido "la etiqueta que se queda antes sin trozos va primero" (una clave más corta que coincide hasta ahí es menor). Ordena con un algoritmo **estable**, para que las etiquetas con claves iguales conserven su orden de entrada.

```
clave(etiqueta) = [(0, len(n), n) para trozos de dígitos, n = trozo sin ceros iniciales ...
                   (1, s)         para trozos de letras]
devolver sorted(labels, key=clave)      # estable
```

### Paso a paso

`box10` → `(1, box), (0, 2, 10)`; `box2` → `(1, box), (0, 1, 2)`; `box2a` → `(1, box), (0, 1, 2), (1, a)`. Así que box2 < box2a < box10. `apple` empieza con `(1, apple)`, menor que `(1, box)`.

**Complejidad:** O(n log n) comparaciones, cada una O(L), más O(n · L) para construir las claves.
