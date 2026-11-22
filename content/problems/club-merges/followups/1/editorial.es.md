## Idea

Usa la estructura union-find del problema base con unión por tamaño: cada raíz guarda cuántos jugadores tiene su club. Cuando `a` y `b` tienen raíces distintas, cuelga el club pequeño del grande y suma los tamaños. Los clubes nunca se separan, así que el más grande solo puede crecer, y solo el club recién formado puede pasar a ser el más grande. Lleva un `biggest` acumulado, que empieza en 1, y actualízalo con el tamaño nuevo después de cada fusión real. No hace falta ningún montículo ni estructura ordenada.

## Complejidad

O((n + m)·α(n)) en tiempo con compresión de caminos y unión por tamaño, donde α es la inversa de la función de Ackermann (como mucho 4 en la práctica); O(n) de memoria extra.
