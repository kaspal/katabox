## Idea

Primero combina cada hoja como en el problema base: ordena por inicio y extiende la última
franja mientras la siguiente empiece en su fin o antes. Cada sala es ahora una lista
ordenada de franjas que no comparten ningún momento. Después recorre las dos listas con
punteros `i` y `j`. Las franjas actuales se solapan en
`[max(a[i].start, b[j].start), min(a[i].end, b[j].end)]` cuando ese inicio no supera ese
fin; guárdalo. La franja que termina antes no puede solaparse con nada posterior de la
otra lista, así que avánzala. Cada trozo guardado queda dentro de una franja de cada sala,
así que los trozos salen ordenados y ya no comparten ningún momento entre sí.

## Complejidad

O(n log n + m log m) en tiempo por ordenar las dos hojas, y después O(n + m) por el
recorrido; O(n + m) de memoria extra para las listas combinadas y la salida.
