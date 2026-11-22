## Idea

La `k`-ésima mejor puntuación hasta ahora es la menor de las `k` mejores actuales, y esas
`k` cambian como mucho en un intercambio por partida: una puntuación nueva supera a la más
débil de ellas o no importa. Guarda las `k` mejores en un montículo de mínimos. Mientras
tenga menos de `k`, añade cada puntuación e informa de `-1` (hasta que se llene). Después,
si la puntuación nueva es mayor que el mínimo del montículo, sustituye el mínimo por ella.
En ambos casos, el mínimo del montículo es la respuesta de esa partida. Los empates no
molestan: cada puntuación igual ocupa su propio sitio en el montículo.

## Complejidad

O(n log k) en tiempo y O(k) de memoria extra, más la salida de tamaño O(n). Como las
puntuaciones son como mucho 300, contar cuántas partidas obtuvieron cada puntuación
también sirve, en O(301) por partida.
