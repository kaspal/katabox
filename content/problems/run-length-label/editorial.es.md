## Enfoque: una pasada por las rachas

Guarda el índice donde empieza la racha actual. Avanza `i` mientras `text[i]` sea igual a la letra de la racha. Cuando cambie (o termine el texto), la racha es `text[start:i]`: emite su letra y su longitud `i - start` si es al menos 2. Después empieza la siguiente racha en `i`.

### Paso a paso

`text = "aaabcc"`:

| racha | longitud | se emite | etiqueta hasta ahora |
| --- | --- | --- | --- |
| aaa | 3 | `a3` | `a3` |
| b | 1 | `b` | `a3b` |
| cc | 2 | `c2` | `a3bc2` |

Las longitudes pueden tener varias cifras: una racha de 100000 `z` se convierte en `z100000`.

**Complejidad:** O(n) en tiempo, y O(n) para la salida. Construir la etiqueta con una lista (o un constructor de cadenas) la mantiene lineal en todos los lenguajes.
