Sumar la duración de cada lámpara cuenta dos veces los segundos que se solapan. Marcar cada segundo como iluminado u oscuro funciona, pero hay hasta 10⁹ segundos.
---
Ordena las lámparas por su hora de inicio. Así las lámparas que se solapan (o se tocan) quedan seguidas, y puedes unirlas en tramos iluminados.
---
Recorre las lámparas ordenadas guardando el tramo actual `[s, e)`. Si la siguiente lámpara empieza en `e` o antes, alarga `e` hasta el final mayor; si no, suma `e − s` al total y empieza un tramo nuevo. No olvides el último tramo.
