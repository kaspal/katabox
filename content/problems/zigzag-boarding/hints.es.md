Buscar el último nodo una y otra vez recorre la lista entera cada vez: O(n²). La mitad trasera se necesita en orden inverso. ¿Puedes prepararla una sola vez?
---
Encuentra el centro con un puntero lento y otro rápido, corta ahí la lista e invierte la segunda mitad.
---
Ahora intercala: toma un nodo de la mitad delantera, uno de la trasera invertida, y así sucesivamente. La mitad delantera tiene los mismos nodos o uno más.
