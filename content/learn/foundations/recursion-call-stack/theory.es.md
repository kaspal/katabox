## La idea

Una función recursiva resuelve un problema llamándose a sí misma sobre partes más pequeñas de él. Aquí `total(a, lo, hi)` suma el tramo `lo..hi`: un solo valor es su propia suma (el **caso base**); un tramo más largo se divide en `mid` y suma los resultados de sus dos mitades. Cada llamada recibe un **marco** en la pila de llamadas con sus propios `lo`, `hi`, `mid`, `left` y `right`. Un marco espera en la pila hasta que vuelven las llamadas que hizo y luego sale. El árbol de la derecha guarda todas las llamadas que se han ejecutado.

## Pasos

1. Llama a `total(a, lo, hi)`: entra un marco nuevo en lo alto de la pila.
2. Si `lo == hi`, devuelve `a[lo]`: el marco sale de la pila y quien llamó continúa.
3. Si no, calcula `mid = (lo + hi) // 2`.
4. Llama a `total(a, lo, mid)` y espera su resultado en `left`.
5. Llama a `total(a, mid + 1, hi)` y espera su resultado en `right`.
6. Devuelve `left + right`; el marco sale de la pila.

## Coste

| Medida | Coste |
|---|---|
| Tiempo | `O(n)`: `2n − 1` llamadas, cada una con trabajo constante |
| Llamadas | `n` casos base más `n − 1` divisiones |
| Profundidad de la pila | `O(log n)`: dividiendo por la mitad se llega a un valor tras unas `log₂ n` divisiones |
| Espacio extra | `O(log n)` marcos a la vez |

## Errores comunes

- **Un caso base que falta o al que nunca se llega.** Si las mitades se solaparan como `lo..mid` y `mid..hi`, el tramo `lo..lo + 1` se llamaría a sí mismo con el mismo tramo para siempre. Cada llamada debe encoger el tramo hasta llegar a `lo == hi`.
- **Desbordar la pila.** Cada llamada en espera ocupa un marco. Quitar un valor cada vez (`a[lo] + total(a, lo + 1, hi)`) da una profundidad `n`, y Python se detiene por defecto hacia los 1 000 marcos. Dividir por la mitad deja la profundidad en `log n`.
- **Creer que las mitades se ejecutan a la vez.** La llamada izquierda termina por completo, con todas sus llamadas, antes de que empiece la derecha. Mira la pila: solo se ejecuta el marco de arriba.

## Cuándo usarlo

La recursión encaja con problemas que se dividen en copias más pequeñas de sí mismos: tramos partidos por la mitad, árboles, datos anidados y búsquedas que prueban una opción y dan marcha atrás. Si la profundidad puede llegar a miles, reescríbela con un bucle y una pila explícita.

## Temas relacionados

- [Torres de Hanói](learn:foundations/towers-of-hanoi): recursión en la que cada llamada hace dos más pequeñas, con `2ⁿ − 1` movimientos.
- [Ordenación rápida](learn:sorting/quick-sort): divide un tramo y recurre en los dos lados.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): de dónde sale `log n`.
- Ordenación por mezcla (próximamente): la misma división por la mitad, mezclando en lugar de sumar.
- Búsqueda binaria (próximamente): dividir por la mitad, pero visitando solo un lado.
