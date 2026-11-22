Quitar emisiones de cualquiera de los dos extremos de un tramo solo puede bajar su número de canciones distintas. ¿Sigue sirviendo una ventana deslizante?
---
Guarda un mapa de canción a cuántas veces aparece en la ventana. El número de canciones distintas es el número de canciones con cuenta mayor que 0.
---
Añade `plays[end]`; mientras el mapa tenga más de `k` canciones, baja la cuenta de `plays[start]`, borra la canción del mapa cuando su cuenta llegue a 0 y avanza `start`. Quédate con la ventana más larga que veas.
