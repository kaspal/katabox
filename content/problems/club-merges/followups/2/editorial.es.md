## Idea

Las vueltas atrás llegan en orden inverso, así que guarda las fusiones en una pila. Usa union-find con unión por tamaño pero **sin** compresión de caminos: así una fusión cambia exactamente dos cosas, el padre de la raíz absorbida `rb` y el tamaño de la raíz a la que se une, y deshacerla consiste en restaurar esas dos. Apila `rb` para una fusión real y una marca (por ejemplo −1) para una fusión dentro de un mismo club, para que las vueltas atrás vayan al paso de las fusiones. Con `[0]`, desapila: en una fusión real, la raíz a la que se unió sigue siendo `parent[rb]`, porque todas las fusiones posteriores ya se han deshecho; réstale `size[rb]`, pon `parent[rb] = rb` y suma 1 al número de clubes. Una pila vacía significa que no hay nada que deshacer. La unión por tamaño mantiene cada árbol con altura de como mucho log₂ n, así que `find` sigue siendo rápido sin compresión.

## Complejidad

O(log n) por fusión (las dos llamadas a `find`) y O(1) por vuelta atrás: O(q log n) en tiempo en total, con O(n + q) de memoria extra.
