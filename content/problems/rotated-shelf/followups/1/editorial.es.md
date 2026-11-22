## Idea

Primero deshaz el giro sin mover nada: la carpeta más pequeña está donde la estantería
"baja", y la búsqueda binaria del problema base la encuentra. Compara `shelf[mid]` con
`shelf[hi]`; si es mayor, la bajada está a la derecha de `mid`. Llama `p` a ese índice;
entonces `s(i) = shelf[(p + i) mod n]` lee la estantería en orden.

Por debajo de `s(i)` hay `s(i) − 1` números positivos, y exactamente `i` de ellos están en
la estantería, así que `free(i) = s(i) − (i + 1)` están libres. `free` nunca baja (dos
carpetas seguidas difieren al menos en 1). Para una pregunta `k`, busca con búsqueda
binaria el primer `i` con `free(i) ≥ k`. El `k`-ésimo número libre está por debajo de
`s(i)` y por encima de `s(i − 1)`, con exactamente `i` carpetas por debajo, así que es
`k + i`. Si ningún `i` sirve, las `n` carpetas están por debajo y la respuesta es `k + n`.

## Complejidad

O(log n) para el punto de giro y O(log n) por pregunta: O(n + q log n) de tiempo con la
entrada, O(1) de memoria extra aparte de la salida. Un conjunto hash de la estantería no
ayuda aquí: las preguntas son sobre números que *no* están en la estantería.
