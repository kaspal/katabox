Recorrer toda la lista para cada prefijo funciona, pero repite las mismas comparaciones hasta 2·10⁸ veces. Las palabras que comparten comienzo deberían encontrarse juntas.
---
Mete las palabras en un **trie**. Todas las palabras que empiezan por un prefijo son justo las que están bajo el nodo del prefijo.
---
Al insertar, guarda en cada nodo la palabra alfabéticamente más pequeña que pasa por él (compárala con la guardada, o inserta las palabras ordenadas para que la primera en llegar sea la menor). Una consulta baja por el prefijo y devuelve esa palabra, o `""` si el camino se corta.
