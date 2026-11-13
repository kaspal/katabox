Probar todas las parejas es O(n²). El XOR se decide bit a bit, y un bit alto vale más que todos los bits inferiores juntos. ¿Puedes elegir los bits con avaricia desde arriba?
---
Para un código `x`, su mejor pareja difiere de él en el bit más alto posible, después en el siguiente, y así sucesivamente. Guarda los códigos en un trie binario (un nivel por bit, el más alto primero) para seguir esa elección.
---
Inserta cada código en el trie; para consultar `x`, baja desde el bit más alto y ve al hijo con el bit **contrario** siempre que exista (ese bit del XOR vale 1). Quédate con la mejor consulta de todos los códigos.
