## Enfoque: ordenar por inicio y barrer

Ordena las reservas por inicio. Recórrelas llevando la franja combinada actual `[s, e]`:

- si la siguiente reserva empieza en `e` o antes (comparten un momento, porque los extremos están incluidos), amplía `e = max(e, fin de la reserva)`;
- si no, la franja actual está terminada: escríbela y empieza otra en esta reserva.

Escribe la última franja al final. Una reserva dentro de otra, como [4, 6] dentro de [3, 20], queda absorbida porque `e` solo crece.

**Complejidad:** O(n log n) por la ordenación, O(n) por el barrido.
