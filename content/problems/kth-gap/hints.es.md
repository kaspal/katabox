Puede haber más de 10⁹ pares, demasiados para listarlos. En vez de buscar la k-ésima distancia directamente, ¿puedes responder una pregunta más fácil: "cuántas distancias son como mucho d"?
---
Con los postes ordenados, los postes a distancia ≤ `d` del poste `j` por su izquierda forman un bloque contiguo que solo se desliza a la derecha al crecer `j`: dos punteros cuentan todas las distancias ≤ d en O(n).
---
El recuento crece con `d`, así que haz búsqueda binaria sobre la respuesta: el menor `d` de `[0, máx − mín]` cuyo recuento es al menos `k` es la k-ésima distancia.
