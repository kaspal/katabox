## Idea

Una pasada de la ventana del problema base da, para cada emisión `r`, `start[r]`: la
primera emisión del tramo sin repeticiones más largo que termina en `r` (el extremo
izquierdo pasa la emisión anterior de la misma canción). Todo tramo sin repeticiones que
termina en `r` empieza en `start[r]` o después, y `start[r]` nunca baja al crecer `r`.

Dentro de una petición `[a, b]`, el mejor tramo que termina en `r` empieza en
`max(start[r], a)`:

- para las emisiones con `start[r] < a` (van todas primero, porque `start` nunca baja), es
  `a..r`, y el más largo está en la última de ellas;
- para las demás, es el tramo entero, de longitud `len[r] = r − start[r] + 1`.

Busca con búsqueda binaria la primera `r0` en `[a, b + 1)` con `start[r0] ≥ a`. La
respuesta es el mayor entre `r0 − a` y el máximo de `len` en `[r0, b]` (cuando `r0 ≤ b`).
Construye una tabla dispersa sobre `len` una sola vez, y cada máximo de un rango cuesta dos
consultas.

## Complejidad

O(n log n) de tiempo y memoria para la tabla dispersa, y luego O(log n) por petición.
Recorrer cada petición entera es O(n) por petición, hasta 10¹⁰ pasos en total.
