## Enfoque: un trie binario, con avaricia desde el bit más alto

Escribe cada código con 20 bits, el más alto primero, e insértalo en un trie cuyos nodos tienen un hijo para el bit 0 y otro para el bit 1.

Para encontrar la mejor pareja de `x`, empieza en la raíz y mira los bits de `x` desde arriba. En cada nivel, prefiere el hijo con el bit contrario: si existe, ese bit del XOR vale 1, y como un bit alto vale más que todos los inferiores juntos, tomarlo siempre es acertado. Si no, sigue por el mismo bit (bit del XOR a 0).

Inserta todos los códigos y después consulta cada uno (o consulta cada código contra los insertados antes): la respuesta es el mayor resultado.

**Variante con conjuntos de prefijos:** la misma avaricia, hecha con conjuntos: construye la respuesta bit a bit desde arriba; para cada bit, reúne los prefijos de todos los códigos y comprueba si dos de ellos dan, con XOR, la respuesta candidata con ese bit a 1.

**Complejidad:** O(n·B) con B = 20 bits, y O(n·B) de espacio para el trie. Probar todas las parejas es O(n²): 5·10⁹ parejas para 10⁵ códigos.
