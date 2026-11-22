Revisar todas las salas para cada reunión puede costar O(n²) cuando hay muchas salas
abiertas. En cada inicio necesitas dos respuestas rápidas: qué salas acaban de quedar
libres y qué sala libre tiene el número más pequeño.
---
Ordena las posiciones de las reuniones por (inicio, posición) y guarda las respuestas
según la posición original. Un montículo de salas ocupadas ordenado por hora de fin te
dice qué salas se liberan antes del inicio actual (o justo en él).
---
Usa dos montículos de mínimos: salas ocupadas como (fin, sala) y números de sala libres.
En cada reunión, pasa al montículo de libres toda sala ocupada con fin ≤ inicio; después
toma el menor número libre, o abre la sala `count + 1` si no hay ninguna libre, y mete
(fin, sala) en el montículo de ocupadas.
