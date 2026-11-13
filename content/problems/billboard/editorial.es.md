## Enfoque: una pila monótona

Todo cartel tiene un panel cubierto más bajo `k`, y para ese `k` el cartel más ancho va desde justo después del panel más bajo más cercano por la izquierda hasta justo antes del panel más bajo más cercano por la derecha. Así que la respuesta es el mejor de n candidatos, uno por panel.

Recorre los paneles de izquierda a derecha con una pila de posiciones cuyas alturas crecen de abajo arriba. Añade al final un panel centinela de altura 0. Para cada posición `i`:

- mientras el panel de la cima sea **más alto** que `panels[i]`, sácalo como `k`: el panel `i` es el más bajo más cercano por su derecha, y el panel que queda en la cima (o el borde izquierdo) es el más bajo o igual más cercano por su izquierda. Su cartel tiene anchura `i − cima − 1` (o `i` con la pila vacía) y área `panels[k] · anchura`;
- mete `i`.

Las alturas iguales no lo estropean: de dos paneles iguales, el posterior saca al anterior solo cuando llega algo más bajo, y entonces el cartel del posterior cubre los dos.

### Paso a paso

`[3, 1, 4, 4, 2, 5]`: el 1 saca al 3 (anchura 1, área 3); el 2 saca los dos 4 (el segundo: anchura 1, área 4; el primero: anchura 2, área 8); el centinela saca el 5 (5), el 2 (anchura 4, área 8) y el 1 (anchura 6, área 6). El mejor es 8.

**Complejidad:** O(n) en tiempo y O(n) de espacio. Probar cada extremo izquierdo con un mínimo acumulado es O(n²).
