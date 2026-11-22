Probar cada enlace girado, con una búsqueda completa cada vez, cuesta
O(links · (n + links)): demasiado lento. ¿Puede una sola búsqueda decidir qué enlace
girar sobre la marcha?
---
Lo que importa en cada momento no es qué enlace giraste, sino si ya has gastado tu único
giro. Haz que eso forme parte de dónde estás: cada estación aparece dos veces, "giro
libre" y "giro gastado".
---
Haz una búsqueda en anchura sobre parejas `(estación, gastado)`, empezando en
`(0, libre)`. Un enlace `[a, b]` en su propio sentido va de `(a, s)` a `(b, s)`; recorrido
al revés va solo de `(b, libre)` a `(a, gastado)`. La respuesta es la menor distancia de
las dos copias de la estación `n − 1` que se alcanzaron.
