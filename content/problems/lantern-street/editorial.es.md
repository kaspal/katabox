## Enfoque: DP sobre el árbol con dos estados

Para cada nodo `v`, calcula en postorden:

- `lit(v)`: el mejor total en el subárbol de `v` cuando `v` está encendida. Sus hijos deben estar apagados: `val(v) + dark(izq) + dark(der)`.
- `dark(v)`: el mejor total cuando `v` no está encendida. Cada hijo es libre: `max(lit(izq), dark(izq)) + max(lit(der), dark(der))`.

Un subárbol vacío tiene `lit = dark = 0`. La respuesta es `max(lit(raíz), dark(raíz))`.

### Paso a paso

Ejemplo 2, `[2, 9, 1, null, null, 7, 8]`: para el 9, lit = 9 y dark = 0; para el 1, lit = 1 y dark = 7 + 8 = 15. En la raíz, lit = 2 + 0 + 15 = 17 y dark = max(9, 0) + max(1, 15) = 24. La respuesta es 24.

**Complejidad:** O(n) en tiempo y O(h) para la recursión. La recursión "mejor(v) = max(val + mejor de los nietos, mejor de los hijos)" sin recordar resultados recalcula subárboles una y otra vez y es exponencial en la altura.
