Tras ordenar las reservas por inicio, ¿qué reservas pueden solaparse con la franja combinada actual?
---
Solo las siguientes en el orden: si la siguiente reserva empieza después de que termine la franja actual, ninguna reserva posterior puede tocarla tampoco.
---
Ordena por inicio. Guarda la última franja combinada; para cada reserva, si su inicio ≤ el fin de la franja, amplía el fin a `max(fin, fin de la reserva)`; si no, empieza una franja nueva.
