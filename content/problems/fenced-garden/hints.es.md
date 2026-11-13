Ocúpate primero de la valla: un punto está sobre un lado cuando está en la recta del lado (producto cruzado cero) y dentro de su caja delimitadora.
---
Para los demás puntos, lanza un rayo desde el punto hacia +x y cuenta cuántos lados cruza. Un número impar significa dentro.
---
Un lado de (x1, y1) a (x2, y2) se cruza cuando exactamente un extremo está por encima del punto (`(y1 > y) != (y2 > y)`) y el cruce está a la derecha del punto. Compara con enteros: el cruce está a la derecha cuando `(y − y1)(x2 − x1) − (x − x1)(y2 − y1)` tiene el mismo signo que `y2 − y1`.
