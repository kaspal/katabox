## Idea

La regla depende de la última pieza, así que guárdala en el estado. Sea `a[i]` el número
de pavimentos de longitud `i` que no terminan en losa larga (con `a[0] = 1` para el
pavimento vacío) y `b[i]` el de los que sí:

- `b[i] = a[i − 3]`: una losa larga solo puede seguir a un pavimento que no termina en
  una.
- `a[i] = (a[i − 1] + b[i − 1]) + (a[i − 2] + b[i − 2])`: una cuadrada o una losa corta
  pueden seguir a cualquier cosa.

La respuesta es `a[n] + b[n]`, todo módulo 1 000 000 007. Solo hacen falta las tres
últimas longitudes, así que bastan unas pocas variables.

## Complejidad

O(n) en tiempo y O(1) de memoria extra (O(n) si guardas los arrays).
