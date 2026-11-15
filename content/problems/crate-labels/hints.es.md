Una etiqueta no tiene espacios, así que dividir toda la entrada por espacios da `N` y luego exactamente un token por etiqueta. Cada etiqueta se divide por `;` en entradas, y cada entrada se divide por `:` en un nombre y una cantidad.
---
Lleva un diccionario (mapa hash) del nombre del producto a su total acumulado. Para cada entrada, suma su cantidad al total del nombre, empezando en 0 la primera vez que aparece.
---
Después de todas las etiquetas, ordena los nombres del diccionario con el orden normal de cadenas de tu lenguaje (con letras minúsculas es el alfabético, y `fig` va antes que `figs`) y escribe `name total` para cada uno, todo de una sola vez.
