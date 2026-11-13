Comprueba cada año de `start` a `end` con un bucle.
---
Escribe la regla como una condición: divisible entre 4, y o bien no divisible entre 100 o bien divisible entre 400.
---
`y % 4 == 0 and (y % 100 != 0 or y % 400 == 0)`: cuenta los años en que se cumple.
