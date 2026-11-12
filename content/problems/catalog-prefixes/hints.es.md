Comparar cada texto escrito con cada título funciona, pero con 2·10⁴ búsquedas sobre 10⁴ títulos son 2·10⁸ comparaciones. ¿Se pueden guardar los títulos de forma que todos los que comparten comienzo se encuentren juntos?
---
Guarda los títulos en un **trie**: un árbol donde cada arista es una letra y cada nodo representa el prefijo que se lee desde la raíz. Cada título es un camino desde la raíz.
---
Al insertar un título, suma 1 a un contador en cada nodo por el que pasas. El contador del nodo de un prefijo es justo el número de títulos que empiezan por él; una búsqueda solo recorre las letras escritas (0 si falta alguna).
