El siguiente cliente de la cola unida siempre está al frente de alguna de las colas. ¿De cuál?
---
Mirar el frente de todas las colas por cada cliente cuesta k comparaciones cada vez (k colas). ¿Hay alguna estructura que dé el menor de k valores enseguida y permita reemplazarlo?
---
Mantén un montículo de mínimos con `(turno del frente, índice de la cola)` de las colas no vacías. Saca el menor, añade ese nodo al resultado y mete el siguiente nodo de la misma cola si lo hay. Cada cliente entra y sale del montículo una sola vez.
