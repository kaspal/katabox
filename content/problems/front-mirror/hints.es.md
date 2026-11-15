Si añades `k` letras delante, ¿qué letras tienen que ser? Compara las `k` primeras letras del resultado con sus `k` últimas. ¿Qué tiene que cumplir entonces el resto de `s`?
---
Las letras añadidas son las últimas letras de `s`, al revés, y lo que queda al principio de `s` tiene que ser un palíndromo. Así que la tarea es encontrar el **palíndromo más largo al principio de `s`**. Comprobar cada prefijo, del más largo al más corto, puede costar unos `n²` pasos; una herramienta de búsqueda en cadenas lo encuentra en una sola pasada.
---
Un prefijo de `s` es un palíndromo exactamente cuando coincide con un sufijo de `reverse(s)`. Construye `t = s + "#" + reverse(s)` y calcula su función de prefijos (KMP): el valor en la última posición, `keep`, es la longitud del prefijo palíndromo más largo (`#` impide que una coincidencia cruce el centro). Escribe `reverse(s[keep:]) + s`.
