## Enfoque: búsqueda binaria usando la mitad ordenada

Busca dentro de `[lo, hi]`, empezando por toda la estantería. Mira el centro `m`:

- si `shelf[m] = label`, has terminado;
- si `shelf[lo] ≤ shelf[m]`, la mitad izquierda `lo..m` está ordenada (el corte entre los dos tramos no está en ella). Si `shelf[lo] ≤ label < shelf[m]`, la etiqueta solo puede estar ahí: `hi = m − 1`; si no, `lo = m + 1`;
- si no, la mitad derecha `m..hi` está ordenada. Si `shelf[m] < label ≤ shelf[hi]`, busca en ella: `lo = m + 1`; si no, `hi = m − 1`.

Cuando `lo > hi`, la etiqueta no está en la estantería.

### Paso a paso

`[41, 57, 63, 8, 15, 22]`, etiqueta 15: m = 2 (63); la mitad izquierda 41..63 está ordenada y 15 no está en [41, 63), así que lo = 3. m = 4 (15): encontrada.

**Complejidad:** O(log n) en tiempo y O(1) de espacio.
