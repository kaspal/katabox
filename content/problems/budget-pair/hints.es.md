Probar todos los pares funciona, pero cuesta O(n²). La lista está ordenada: ¿cómo puedes aprovecharlo?
---
Empieza por el artículo más barato y el más caro. Si su total supera el presupuesto, ¿puede el más caro formar un par válido con alguien más barato que el más barato?
---
Mantén dos índices, `lo` al principio y `hi` al final. Si `prices[lo] + prices[hi]` cabe, anótalo y mueve `lo` a la derecha (quizá cabe una pareja mayor); si no, mueve `hi` a la izquierda. Para cuando se crucen.
