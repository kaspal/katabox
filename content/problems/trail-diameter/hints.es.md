Una búsqueda en anchura (BFS) desde un cruce da la distancia (en senderos) a todos los demás. Lanzarla desde cada cruce encuentra la respuesta, pero cuesta O(N²): 4·10⁸ pasos con 20 000 cruces.
---
Empieza una BFS en cualquier cruce y quédate con el cruce `u` más lejano. En un árbol, `u` siempre es un extremo de alguna ruta más larga.
---
Lanza una segunda BFS desde `u`: la mayor distancia que encuentre es la respuesta. Construye las listas de adyacencia una vez y usa una cola (un arreglo con un índice de cabeza), no recursión, para que una cadena de 20 000 cruces no sea un problema.
