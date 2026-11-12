## Enfoque: llenar cada línea con avidez

Las reglas describen una sola pasada: guarda las palabras de la línea actual y su longitud, y para cada palabra decide si se suma a la línea o empieza una nueva.

```
lineas = []; actual = [words[0]]; longitud = len(words[0])
para w en words[1:]:
    si longitud + 1 + len(w) <= width:     # cabe tras un espacio
        actual.añadir(w); longitud += 1 + len(w)
    si no:
        lineas.añadir(" ".join(actual))
        actual = [w]; longitud = len(w)    # una palabra larga también empieza línea
lineas.añadir(" ".join(actual))
```

Una palabra más larga que `width` nunca cabe tras otra palabra, así que empieza una línea, y la siguiente palabra tampoco cabe tras ella: queda sola, como se pide. Unir las palabras de una línea una sola vez al final mantiene el trabajo lineal.

### Paso a paso

Ancho 10: "the" (3), "quick" cabe (9), "brown" no (15) → línea "the quick". "brown" (5), "fox" cabe (9), "jumps" no (15) → "brown fox". La última línea es "jumps".

**Complejidad:** O(total de caracteres) de tiempo y de tamaño de la salida.
