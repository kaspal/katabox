Comparar cada reserva de una sala con cada reserva de la otra es O(n · m), y los solapes
dentro de cada hoja complican los trozos. ¿Puedes ordenar cada hoja primero?
---
Combina cada hoja por separado, igual que en el problema base. Ahora cada sala es una
lista ordenada de franjas que no comparten ningún momento. Dos franjas `[s1, e1]` y
`[s2, e2]` se solapan en `[max(s1, s2), min(e1, e2)]` cuando ese inicio no supera ese fin.
---
Recorre las dos listas combinadas con un puntero cada una. Guarda el solape de las dos
franjas actuales si no está vacío y después avanza la franja que termina antes
(cualquiera de las dos si terminan a la vez).
