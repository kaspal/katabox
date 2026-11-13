## Enfoque: cuatro orientaciones

Sea `o(A, B, C)` el signo del producto cruzado `(B − A) × (C − A)`: +1 si C está a la izquierda de A→B, −1 si está a la derecha y 0 si está en la recta.

Calcula `d1 = o(P1, P2, P3)`, `d2 = o(P1, P2, P4)`, `d3 = o(P3, P4, P1)` y `d4 = o(P3, P4, P2)`.

1. **Cruce propio:** si `d1` y `d2` son no nulos y de signos opuestos, y también `d3` y `d4`, los senderos se cruzan en un punto interior.
2. **Contacto o solapamiento:** si no, los senderos solo se encuentran si algún extremo está **sobre** el otro sendero: su orientación es 0 y está dentro de la caja delimitadora del otro sendero. Por ejemplo, `d1 = 0` y P3 dentro de la caja de P1, P2.
3. Si no, no se encuentran.

Los senderos colineales quedan cubiertos por el paso 2: se solapan exactamente cuando un extremo de uno está dentro del otro.

**Complejidad:** O(1) por pareja, todo en enteros.
