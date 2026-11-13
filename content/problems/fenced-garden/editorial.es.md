## Enfoque: comprobar la valla y después lanzar un rayo

**¿Sobre la valla?** Para cada lado A→B, el punto P está sobre él cuando `cross(A, B, P) = 0` (P está en la recta) y P está dentro de la caja delimitadora de A y B. Comprobar primero todos los lados resuelve el caso `"boundary"` con exactitud.

**¿Dentro o fuera?** Lanza un rayo horizontal desde P hacia +x y cuenta los lados que cruza. Los puntos interiores lo cruzan un número impar de veces. Para contar cada esquina una sola vez, considera que un lado cruza el rayo solo cuando un extremo está estrictamente por encima de P y el otro no: `(A.y > P.y) != (B.y > P.y)`. Esa regla semiabierta cuenta una vez cada esquina compartida por dos lados e ignora los lados horizontales.

La coordenada x del cruce es `A.x + (P.y − A.y)(B.x − A.x)/(B.y − A.y)`. Para seguir en enteros, multiplica: el cruce está a la derecha de P cuando `t = (P.y − A.y)(B.x − A.x) − (P.x − A.x)(B.y − A.y)` es positivo y `B.y > A.y`, o negativo y `B.y < A.y`.

### Paso a paso

El punto (3, 3) en la valla con muesca del ejemplo 1: ¿el rayo hacia la derecha cruza el lado de (6, 4) a (3, 2)? Sus extremos están en y = 4 e y = 2, así que abarca y = 3, y el cruce está en x = 4,5 > 3: un cruce. El lado (6, 0)→(6, 4) también se cruza. Dos cruces: fuera.

**Complejidad:** O(n) por punto, O(n·m) en total; con n ≤ 300 y m ≤ 1000, son 3·10⁵ pasos.
