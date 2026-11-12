La maquetación se decide palabra a palabra, de izquierda a derecha: solo hace falta saber cuánto mide ya la línea actual.
---
Guarda las palabras de la línea actual y su longitud. Una palabra nueva cabe cuando `longitud + 1 + len(palabra) <= width` (el 1 es el espacio); la primera palabra de una línea siempre entra, mida lo que mida.
---
Cuando una palabra no cabe, une las palabras actuales con un espacio, añade esa línea al resultado y empieza una línea nueva con la palabra. Al final, añade la última línea.
