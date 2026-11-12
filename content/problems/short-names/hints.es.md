Un comienzo `p` de un nombre sirve de etiqueta exactamente cuando **ningún otro** nombre empieza por `p`, es decir, cuando solo un nombre (este) empieza por `p`. Comparar cada nombre con todos los demás cuesta O(n²·L).
---
Mete todos los nombres en un **trie** y cuenta, en cada nodo, cuántos nombres pasan por él. Esa cuenta es el número de nombres que empiezan por el prefijo del nodo.
---
Para cada nombre, baja por sus letras; el primer nodo con cuenta 1 marca el final de la etiqueta. Si llegas al final del nombre sin encontrarlo, la etiqueta es el nombre entero.
