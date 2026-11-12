Pulsar el botón tantas veces como canciones tiene la lista no cambia nada. ¿Cuántas de las `k` pulsaciones importan de verdad?
---
Tras `r = k mod n` pulsaciones (n canciones), las últimas `r` canciones quedan delante, en su orden, seguidas de las demás. ¿Qué canción queda la última, y cuál la primera?
---
Recorre la lista una vez para contar `n` y encontrar la cola. La nueva última canción es la que está `n − r − 1` pasos después de la cabeza; la siguiente pasa a ser la nueva cabeza. Corta el enlace ahí y cuelga la cabeza antigua detrás de la cola antigua.
