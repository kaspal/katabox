## Enfoque: enlazar el frente menor

Usa un nodo ficticio `head` y un puntero `tail` que apunte a él. Mientras las dos colas tengan nodos, compara sus frentes, enlaza el menor tras `tail`, mueve `tail` a él y avanza esa cola. En un empate vale cualquiera de los dos frentes (aquí el de la primera cola), porque solo importan los números. Cuando una cola se vacía, cuelga de golpe los nodos que le quedan a la otra de `tail`. La respuesta es `head.next`.

No se crea ni se copia ningún nodo: el resultado reutiliza los nodos de entrada.

**Complejidad:** O(n + m) en tiempo y O(1) de espacio extra.
