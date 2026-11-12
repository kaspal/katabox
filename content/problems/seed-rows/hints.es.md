Listar las plantaciones una a una no tiene remedio: puede haber muchísimas. Cuéntalas según cómo **terminan**: de qué clase es la última planta y cuánto bancal ocupa.
---
Sea `formas[x][k]` el número de plantaciones válidas de los primeros `x` centímetros cuya última planta es de la clase `k`. Quitar esa última planta deja una plantación válida de `x − widths[k]` centímetros que **no** termina en la clase `k` (o el bancal vacío).
---
Así que `formas[x][k] = total[x − w] − formas[x − w][k]` con `w = widths[k]`, donde `total[y]` es la suma sobre todas las clases y `total[0] = 1` (el bancal vacío). La respuesta es `total[length]`, todo módulo 1 000 000 007.
