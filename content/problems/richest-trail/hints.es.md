Toda ruta tiene una cueva más alta donde se dobla. Fija esa cueva: la mejor ruta que se dobla ahí baja por el mejor camino a cada lado.
---
Para cada nodo, calcula `abajo(v)`: el mejor total de una ruta que empieza en v y solo baja. Una rama con total negativo es mejor no tomarla.
---
`abajo(v) = val + max(0, abajo(izq), abajo(der))`. Una ruta que se dobla en v vale `val + max(0, abajo(izq)) + max(0, abajo(der))`; quédate con la mayor de todos los nodos. Empieza la respuesta en −∞, no en 0, porque todos los valores pueden ser negativos.
