Fíjate en la última pieza de cualquier pavimento: es una cuadrada, una losa corta o una losa larga. ¿Qué queda delante?
---
Si la última pieza es una cuadrada, el resto es cualquier pavimento de un sendero de longitud `n − 1`; una losa corta deja `n − 2` y una larga `n − 3`. Así que el número de pavimentos de `n` es la suma de los números para `n − 1`, `n − 2` y `n − 3`.
---
Rellena una tabla desde el principio: `formas[0] = 1` (el sendero vacío, pavimentado de una forma) y `formas[i] = formas[i−1] + formas[i−2] + formas[i−3]` (los términos que faltan valen 0), tomando el resto cada vez. La recursión sin tabla recalcula las mismas longitudes una y otra vez.
