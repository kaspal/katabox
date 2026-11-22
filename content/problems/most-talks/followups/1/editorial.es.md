## Idea

Con puntuaciones, la estrategia voraz del problema base (la que termina antes) falla, así
que usa programación dinámica sobre las charlas ordenadas por fin. Sea `best[i]` la mejor
puntuación total usando solo las primeras `i` charlas en ese orden (`best[0] = 0`). La
charla `i` con `[s, e, score]` se salta, y queda `best[i]`, o se toma. Si se toma, todas
las demás charlas elegidas deben terminar en `s` o antes; como los finales están
ordenados, esas charlas forman un prefijo, y una búsqueda binaria (la cota superior de `s`
entre los finales anteriores) da su longitud `j`. Así
`best[i + 1] = max(best[i], best[j] + score)`, y la respuesta es la última entrada.

## Complejidad

O(n log n) en tiempo por la ordenación y las búsquedas binarias, O(n) de memoria extra. El
total es como mucho 10⁵ · 10⁴ = 10⁹, que cabe en un entero de 32 bits.
