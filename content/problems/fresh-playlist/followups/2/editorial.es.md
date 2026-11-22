## Idea

La misma ventana deslizante del problema base, con otra regla que restaurar. Mueve `end` por las emisiones, contando cada canción dentro de `[start, end]` en un mapa hash, y borra una canción del mapa cuando su cuenta baje a 0, para que el tamaño del mapa sea el número de canciones distintas. Cuando añadir `plays[end]` hace que el tamaño pase de `k`, avanza `start`, quitando emisiones, hasta que vuelva a ser `k`. La ventana es entonces el tramo válido más largo que acaba en `end`; quédate con el mayor. Cuando `k` es al menos el número de canciones distintas del registro, la respuesta es el registro entero.

## Complejidad

O(n) en tiempo, porque cada emisión entra y sale de la ventana una vez; O(min(n, k)) de memoria extra para el mapa.
