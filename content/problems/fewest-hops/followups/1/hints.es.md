Enumerar todas las rutas y quedarse con las más cortas explota: una red de unas pocas
decenas de estaciones ya puede tener millones de rutas más cortas. ¿Cómo es una ruta más
corta a una estación justo antes de su último salto?
---
Una ruta más corta a la estación `v`, a distancia `d`, es una ruta más corta a algún vecino
`u` a distancia `d − 1`, más el salto `u → v`. Así que el número de rutas a `v` es la suma
de los números de rutas a esos vecinos.
---
Haz el recorrido en anchura del problema base desde la estación `0` con un arreglo `ways`
(`ways[0] = 1`). Cuando encuentres un vecino por primera vez, fija su distancia y copia
`ways[u]`; cuando lo vuelvas a encontrar a la misma distancia `d[u] + 1`, suma `ways[u]`.
Reduce módulo 1 000 003.
