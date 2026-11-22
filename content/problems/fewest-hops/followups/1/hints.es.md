Repetir la búsqueda del problema base una vez por estación costaría O(n · (n + links)):
demasiado lento para 10⁵ estaciones. ¿No había aprendido ya la búsqueda base más de lo que
devolvía?
---
Una búsqueda en anchura desde la estación `0` llega a las estaciones en orden de
distancia, y encuentra el menor número de saltos a cada estación que visita, no solo a
`n − 1`.
---
Haz una sola búsqueda en anchura desde la estación `0` por los dos sentidos de cada
enlace, y sigue hasta vaciar la cola en lugar de parar antes. Anota la distancia de cada
estación la primera vez que la ves; las estaciones que nunca ves se quedan con `-1`.
