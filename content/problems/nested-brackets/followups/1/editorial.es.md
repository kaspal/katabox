## Idea

En el problema base un solo corchete mal puesto daba `false`; aquí cada paréntesis suelto tiene su propio arreglo. Recorre de izquierda a derecha con `open`, el número de `(` aún sin cerrar. Un `)` cierra uno de ellos si `open > 0`; si no, ningún paréntesis a su izquierda podrá emparejarlo, así que hay que insertar un `(` (justo antes sirve). Al final, cada uno de los `open` que siguen esperando necesita su propio `)`. Cada inserción arregla exactamente un paréntesis suelto y cada paréntesis suelto necesita una, así que `added + open` es el mínimo.

## Complejidad

O(n) en tiempo y O(1) de memoria extra: con un solo tipo de paréntesis, un contador sustituye a la pila.
