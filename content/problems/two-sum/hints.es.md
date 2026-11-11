Probar todos los pares funciona, pero cuesta O(n²). ¿Qué te gustaría saber de los números que ya recorriste?
---
Para cada número x, la pareja que buscas es target − x. ¿Puedes comprobar en O(1) si ya apareció?
---
Guarda en un mapa hash cada valor con su índice mientras recorres el arreglo. Busca la pareja antes de guardar el número actual.
