## Enfoque: ordenar por (distancia al cuadrado, índice) con un montículo

Usa la clave `(x² + y², i)` para el puesto `i`: las distancias al cuadrado se comparan igual que las distancias, son exactas en enteros (como mucho 2·10⁸) y el índice resuelve los empates.

Mantén un **montículo de máximos** con como mucho k claves. Para cada puesto, mete su clave; si el montículo pasa de k, saca la mayor. Al final el montículo tiene las k claves menores; ordénalas y devuelve sus índices.

Ordenar las n claves y tomar las k primeras también es correcto, en O(n log n).

**Complejidad:** O(n log k) en tiempo y O(k) de espacio extra. Seleccionar el mínimo k veces recorriéndolo todo es O(n·k): 5·10⁹ pasos para n = 10⁵ y k = n / 2.
