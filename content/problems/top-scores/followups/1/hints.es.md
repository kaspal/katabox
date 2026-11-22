Ordenar las partidas de nuevo tras cada una cuesta O(n² log n). ¿Qué puntuaciones no pueden
volver a ser la `k`-ésima mejor una vez que salen de las `k` mejores?
---
Una puntuación que sale de las `k` mejores no vuelve: las partidas nuevas solo la empujan
más abajo. Así que basta con guardar las `k` mejores actuales, y la `k`-ésima es la menor
de ellas.
---
Guarda un montículo de mínimos con `k` puntuaciones como mucho. Añade mientras tenga menos
de `k`; después, sustituye la cima cuando una puntuación nueva la supere. Tras cada
partida, informa de la cima cuando el montículo tenga `k` puntuaciones, y `-1` antes.
