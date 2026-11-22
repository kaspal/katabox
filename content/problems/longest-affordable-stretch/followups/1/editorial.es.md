## Idea

Los costes negativos rompen la ventana de dos punteros, así que pasa a sumas prefijas. Las
etapas `i..j−1` cuestan `prefix[j] − prefix[i]` y caben cuando
`prefix[i] ≥ prefix[j] − budget`; para cada final `j`, el mejor inicio es el `i < j` más
temprano que lo cumple. Si un inicio anterior ya tiene un prefijo al menos igual de alto
que otro posterior, el posterior nunca gana, así que guarda solo los inicios que marcan un
nuevo máximo del prefijo, empezando por `prefix[0] = 0`. Sus prefijos son crecientes, así
que una búsqueda binaria encuentra el primero ≥ `prefix[j] − budget`, que es también el
inicio válido más temprano. Actualiza la mejor longitud con `j − i` y después guarda
`prefix[j]` si es un nuevo máximo. Si ningún inicio cabe nunca, la respuesta se queda en 0.

## Complejidad

O(n log n) en tiempo y O(n) de memoria extra. Las sumas prefijas quedan dentro de ±5·10⁸,
así que bastan enteros de 32 bits, aunque `prefix[j] − budget` puede llegar a unos 1,5·10⁹.
