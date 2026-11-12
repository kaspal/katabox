## Enfoque: marcadores acumulados iguales

Define el marcador acumulado `s[0] = 0` y `s[j] = s[j − 1] + (+1 por W, −1 por L, 0 por D)`. Una racha que cubre los partidos `i + 1 … j` tiene victorias − derrotas = `s[j] − s[i]`, así que está equilibrada exactamente cuando `s[i] == s[j]`. La respuesta es el número de parejas `i < j` de posiciones con marcadores iguales.

```
visto = {0: 1}; marcador = 0; cuenta = 0
para c en results:
    marcador += 1 si c == "W", −1 si c == "L", 0 si no
    cuenta += visto.get(marcador, 0)    # cada marcador igual anterior cierra una racha
    visto[marcador] = visto.get(marcador, 0) + 1
```

### Paso a paso

"WWLL": marcadores 0, 1, 2, 1, 0. Las parejas de marcadores iguales son (1, 1), que es "WL" en el centro, y (0, 0), que es la cadena entera. Cuenta 2.

**Complejidad:** O(n) de tiempo y O(n) de memoria para el mapa. La respuesta puede llegar a unos 5·10⁹ (una temporada de empates), así que hacen falta enteros de 64 bits. Revisar cada racha cuesta O(n²).
