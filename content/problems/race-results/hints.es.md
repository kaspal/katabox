Las entradas en las que todos los tiempos son distintos salen bien. Prueba con dos corredores con el mismo tiempo.
---
Ordenar por tiempo funciona. Fíjate en cómo se deshacen los empates.
---
La primera ordenación pone los nombres con `reverse=True`, y la ordenación estable por tiempo conserva ese orden descendente en los empates. Quita `reverse=True` (en TypeScript, el desempate devuelve `1` y `-1` al revés).
