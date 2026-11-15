Revisar cada tramo son unos `n²/2` tramos, demasiados para `n = 10⁵`. "Exactamente `K`" es incómodo de contar directamente; ¿es más fácil "como mucho `K`"?
---
Si un tramo tiene como mucho `m` fuentes distintas, también las tiene cualquier tramo más corto dentro de él. Así que, para cada segundo final, los inicios válidos forman un bloque que termina en ese segundo, y su borde izquierdo solo avanza hacia la derecha cuando avanza el final: una ventana deslizante. Luego **exactamente `K` = como mucho `K` − como mucho `K − 1`**.
---
`atMost(m)`: guarda un contador por id en un mapa hash (los ids llegan a 10⁹, así que no sirve un arreglo). Para cada `right`, añade `ids[right]`; mientras el mapa tenga más de `m` ids, quita `ids[left]` (bórralo cuando su contador llegue a 0) y avanza `left`; después suma `right − left + 1` al total. Escribe `atMost(K) − atMost(K − 1)`, con `atMost(0) = 0`.
