Si conoces la profundidad de cada persona (su generación), la distancia es prof(a) + prof(b) − 2 · prof(c), donde c es el antepasado común más cercano. Todo el problema consiste en encontrar c deprisa.
---
Subir de progenitor en progenitor puede costar 10⁵ pasos por pareja cuando el árbol es una cadena larga. ¿Y si cada persona conociera también a su antepasado 2, 4, 8, … generaciones más arriba?
---
Construye una tabla `up[k][v]`: el antepasado 2ᵏ generaciones por encima de v (la fundadora apunta a sí misma). Sube a la persona más profunda hasta la profundidad de la otra usando los dígitos binarios de la diferencia; si ahora son la misma persona, esa es c. Si no, desde la k más grande hacia abajo, haz saltar a las dos siempre que `up[k]` sea distinto para ellas; al final, su progenitor es c.
