El perfil solo puede cambiar donde algún edificio empieza o termina. ¿Cuántas posiciones así hay, y qué necesitas en cada una?
---
Barre las posiciones de izquierda a derecha. En cada una necesitas el edificio más alto que ya empezó y aún no terminó. ¿Qué estructura da el máximo mientras los edificios entran y salen?
---
Un montículo de máximos de `(altura, right)`. En la posición `x`: mete cada edificio con `left ≤ x`; saca de la cima mientras su `right ≤ x` (los edificios terminados que estén más abajo pueden esperar a llegar a la cima). La altura de la cima, o 0, es el perfil en `x`: anótala si cambió.
