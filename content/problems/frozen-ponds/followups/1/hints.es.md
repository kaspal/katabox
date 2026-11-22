Una celda sola necesita 4 lados de valla. ¿Qué le pasa al total cuando dos celdas heladas de un estanque están una junto a otra?
---
Explora cada estanque como en el problema base, con una pila o una cola desde cada `#` sin visitar. Mientras visitas una celda, mira sus cuatro vecinas.
---
Por cada uno de los cuatro lados, suma 1 a la valla del estanque si la vecina está fuera del mapa o es `.`; si no, visita la vecina si es nueva. Junta un total por estanque y ordena los totales de mayor a menor.
