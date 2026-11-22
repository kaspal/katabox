## Idea

Dar la vuelta equivale a leer la temporada dos veces seguidas. Mantén la pila del problema base con los días que esperan un mediodía más cálido, el más frío arriba, y recorre `k` de 0 a `2n − 1` sobre el día `i = k mod n`. Mientras el día `d` de arriba sea más frío que el día `i`, desapílalo y pon su espera en `(i − d) mod n`. Apila días solo en la primera vuelta, para que cada día entre una sola vez; la segunda vuelta solo responde a los días que siguen esperando. Un día que tiene algún día más cálido lo encuentra en los `n − 1` pasos siguientes, así que los días que quedan en la pila son justo los de la temperatura máxima, y se quedan con `0`.

## Complejidad

O(n) en tiempo: 2n pasos, y cada día se apila y desapila como mucho una vez. O(n) de memoria extra para la pila.
