Una rama solo se puede juzgar después de sus subramas, así que importa el orden del recorrido. ¿Qué recorrido visita los hijos antes que su padre?
---
Para cada rama necesitas el total de lo que queda debajo de ella tras los cortes de allí. ¿Puede cada llamada devolver ese total junto con la subrama podada?
---
Postorden: poda primero los hijos izquierdo y derecho, y que cada uno devuelva (lo que queda, su total). El total de la rama es su valor más esos totales. Si es negativo, devuelve (nada, 0); si no, engancha los hijos podados y devuelve (la rama, el total).
