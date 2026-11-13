## Enfoque: un registro deslizante por usuario

Guarda un mapa de cada usuario a una cola (deque) con los instantes de sus peticiones **aceptadas**, de la más antigua a la más reciente.

Para una operación del usuario `u` en el instante `t`:

1. quita del principio de la cola de `u` los instantes `≤ t − window`; están fuera de la ventana `(t − window, t]` y, como los instantes nunca disminuyen, nunca vuelven;
2. `"count"`: informa la longitud de la cola;
3. `"request"`: si la longitud es menor que `limit`, informa 1 y mete `t`; si no, informa 0 (una petición rechazada no se guarda).

Cada petición aceptada entra una vez y sale como mucho una vez, así que el trabajo total es lineal. Cada cola guarda como mucho `limit` instantes.

### Paso a paso

Ejemplo 1: la cola del usuario 1 pasa a [1] y [1, 3]; en el instante 5 está llena, así que 0; en el instante 11 sale el 1 (1 ≤ 11 − 10) y queda [3], así que la petición se acepta: [3, 11]. El usuario 2 empieza vacío.

**Complejidad:** O(1) amortizado por operación y O(usuarios · limit) de espacio.
