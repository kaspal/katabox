Cada fila por sí sola está ordenada. ¿Cuál es la búsqueda más sencilla que lo aprovecha?
---
Una búsqueda binaria en cada fila es O(filas · log columnas). ¿Puede una sola comparación descartar una fila entera o una columna entera?
---
Empieza en el asiento de arriba a la derecha. Si es mayor que la entrada, todo lo que tiene debajo en su columna también lo es: descarta la columna (muévete a la izquierda). Si es menor, todo lo que tiene a su izquierda en su fila también lo es: descarta la fila (baja). Si es igual, la has encontrado.
