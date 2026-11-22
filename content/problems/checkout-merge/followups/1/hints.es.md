Si ya sabes juntar las colas en orden, los números repetidos son fáciles de detectar.
¿Dónde quedan los números de turno iguales en una fusión ordenada?
---
En el orden fusionado, todas las copias de un número salen una detrás de otra. Solo
necesitas recordar el último número que añadiste al resultado.
---
Fusiona con un montículo de mínimos que guarde el primero de cada cola (valor, índice de
la cola, nodo). Saca el menor, mete su siguiente y añade su valor solo si es distinto del
último valor añadido.
