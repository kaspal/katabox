Volver a contar las islas con un relleno por inundación después de cada casilla cuesta toda la tierra cada vez. Una casilla nueva solo cambia lo que tiene alrededor: ¿cuánto puede cambiar la cuenta?
---
Una casilla nueva es una isla nueva (+1). Después, cada isla **distinta** entre sus cuatro vecinas de lado se une a ella (−1 por cada una). Necesitas responder rápido a "¿están ya estas dos casillas en la misma isla?": union-find.
---
Numera las casillas como `r·(C + 2) + c`, con un borde de mar alrededor para que las vecinas nunca se salgan del mapa. Guarda `parent[]`, con −1 para el mar. Para una casilla nueva: `parent[cell] = cell`, `islands += 1`; para cada vecina de tierra, busca las dos raíces (un bucle que acorta el camino a la mitad) y, si son distintas, enlaza una con la otra y haz `islands −= 1`. Una casilla que ya es tierra escribe la cuenta sin cambios.
