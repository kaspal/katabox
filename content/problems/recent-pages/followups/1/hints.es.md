Recorrer todas las páginas de la caché para elegir cuál quitar cuesta O(capacity) en cada
expulsión, demasiado lento para 2·10⁵ operaciones con una caché grande. ¿Qué te dejaría
encontrar la víctima al instante?
---
Agrupa las páginas por su número de usos. Dentro de un grupo, el truco del problema base
(un orden por último uso) te dice qué página es la más antigua.
---
Guarda, para cada número de usos, las páginas con ese número en orden de último uso, y
además el menor número que tiene alguna página. Un uso mueve una página del grupo `f` al
final del grupo `f + 1` (si el grupo `f` se vacía y era el menor, el menor pasa a ser
`f + 1`); una página nueva entra en el grupo 1, que pasa a ser el menor; para expulsar,
quita la página más antigua del grupo menor.
