## Enfoque: alturas de abajo arriba y el mejor giro en cada nodo

Sea `prof(v)` el número de nodos del camino descendente más largo que empieza en `v` (0 para un subárbol vacío). Todo paseo tiene un nodo más alto, donde gira; un paseo que gira en `v` tiene como mucho `prof(izq) + prof(der)` caminos, y se alcanza.

Un recorrido en postorden calcula `prof(v) = 1 + max(prof(izq), prof(der))` y, a la vez, actualiza `mejor = max(mejor, prof(izq) + prof(der))`.

### Paso a paso

Ejemplo 2: en el nodo 2, el lado izquierdo (3 → 5) tiene profundidad 2 y el derecho (4 → 6) también, así que un paseo que gira en 2 tiene 4 caminos. En la raíz, lo mejor es la profundidad del subárbol del 2 = 3 por un lado y 0 por el otro: 3. La respuesta es 4.

**Complejidad:** O(n) en tiempo y O(h) para la recursión.
