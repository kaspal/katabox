La lista ya está ordenada y limpia. ¿Qué franjas puede tocar la nueva?
---
Las franjas se dividen en tres grupos: las que terminan antes de que empiece `fresh` (intactas, a la izquierda), las que comparten un momento con ella y las que empiezan después de que termine (intactas, a la derecha).
---
Copia el grupo de la izquierda, absorbe en `fresh` cada franja con `inicio ≤ fin de fresh` (toma el menor inicio y el mayor fin), añade la franja crecida y copia el resto.
