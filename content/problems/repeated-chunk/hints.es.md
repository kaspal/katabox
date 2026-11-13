Si un trozo de longitud L aparece dos veces, ¿qué pasa con los trozos de longitud L − 1?
---
También aparecen dos veces (quita la última letra de las dos copias). Así que "¿hay un trozo repetido de longitud L?" pasa de sí a no en la respuesta: búsqueda binaria sobre L.
---
Para comprobar un L en O(n), da a cada trozo de longitud L un número con un **hash rodante**: a partir del hash de `text[i..i+L)` obtienes el de `text[i+1..i+L+1)` en O(1). Dos hashes iguales (con dos módulos independientes, para que las coincidencias falsas sean prácticamente imposibles) indican una repetición.
