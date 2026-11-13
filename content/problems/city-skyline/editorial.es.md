## Enfoque: una línea de barrido con un montículo de máximos

El perfil es constante entre **posiciones críticas** consecutivas, los `left` y `right` de todos los edificios. Ordena los edificios por `left` y las posiciones críticas distintas de menor a mayor, y barre:

- mete en un montículo de máximos por altura cada edificio con `left ≤ x`, recordando su `right`;
- mientras la cima del montículo tenga `right ≤ x`, ya no cubre `x`: sácala. Los edificios terminados que no están en la cima se quedan en el montículo; no molestan hasta que llegan a la cima, y entonces se sacan (borrado perezoso);
- el perfil en `x` es la altura de la cima, o 0 con el montículo vacío. Si es distinta de la altura anterior, anota `[x, altura]`.

### Paso a paso

Ejemplo 1: en 1, el edificio de 3 → [1, 3]; en 2 entra el de 5 → [2, 5]; en 4 termina el de 3 pero no está en la cima, sigue 5; en 6 termina el de 5 y después se saca el de 3 (terminado) → [6, 0]; en 8 → [8, 2]; en 9 → [9, 0].

**Complejidad:** O(n log n) en tiempo y O(n) de espacio. Calcular el perfil en cada posición crítica recorriendo todos los edificios es O(n²).
