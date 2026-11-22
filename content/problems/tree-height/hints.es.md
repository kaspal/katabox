Empieza por los árboles más pequeños: ¿qué devuelve el código para `[]` y para un solo nodo `[7]`? ¿Falla en todos los árboles o solo en algunos?
---
El paso recursivo "uno por este nodo más el subárbol más alto" está bien. Mira qué devuelve la función cuando no hay ningún nodo.
---
El caso base devuelve 1 para un nodo que falta, así que cada camino suma un nivel fantasma al final. Un árbol vacío tiene altura 0: devuelve 0 cuando `root` es `None` (`null` en TypeScript).
