## Idea

Un montículo de mínimos con las `k` mejores (el truco de la clasificación en directo) no
puede quitar una partida que sale de la ventana, así que cuenta por valor. Las
puntuaciones van de 0 a 300: guarda `count[v]`, cuántas partidas de la ventana actual
obtuvieron `v`. Mover la ventana una partida suma 1 a la cuenta de la puntuación nueva y
resta 1 a la de la que sale. La `k`-ésima más alta es el valor en el que la suma acumulada
de cuentas desde 300 hacia abajo alcanza `k` por primera vez. Recorrer 301 cuentas por
ventana ya es bastante rápido; un árbol de Fenwick sobre los valores lo encuentra en
O(log 301) buscando la `(w − k + 1)`-ésima menor.

## Complejidad

O(n · log V) en tiempo con un árbol de Fenwick (u O(n · V) con un recorrido simple), donde
V = 301, y O(V) de memoria extra aparte de la salida de `scores.length − w + 1` valores.
