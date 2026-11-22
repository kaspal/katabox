## Idea

Sea `prefix[j]` el total de los primeros `j` días. Un tramo de los días `i..j−1` suma
`prefix[j] − prefix[i]`, y tiene de 1 a `k` días exactamente cuando `j − k ≤ i ≤ j − 1`.
Así que, para cada final `j`, el mejor tramo resta el menor prefijo de esa ventana
deslizante de inicios. Una cola doble monótona da ese mínimo: antes de añadir el inicio
`j − 1`, saca por el fondo cada índice cuyo prefijo no sea menor, porque el nuevo inicio es
posterior y al menos igual de bajo; después quita el frente si ha salido de la ventana. El
frente es el mínimo, y el mejor `prefix[j] − prefix[frente]` sobre todos los `j` es la
respuesta. Empezar el mejor con el primer tramo (y no con 0) mantiene bien los casos en que
todos los días dan pérdidas.

## Complejidad

O(n) en tiempo, porque cada índice entra y sale de la cola una vez, y O(n) de memoria extra
para los prefijos y la cola. Los totales quedan dentro de ±2·10⁹, que cabe en un entero de
32 bits con signo, aunque usar aritmética de 64 bits es la costumbre segura.
