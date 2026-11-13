Sumar los operarios a cada tramo de cada obra son hasta 10⁵ × 10⁵ pasos. ¿Se puede anotar una obra con una cantidad de trabajo constante?
---
En lugar de los recuentos, anota dónde **cambian**: una obra suma `crews` a partir de `a` y deja de sumar después de `b`.
---
Guarda `change[i]`: `change[a] += crews` y `change[b + 1] −= crews`. Una suma acumulada de `change` desde la izquierda da el recuento de cada tramo.
