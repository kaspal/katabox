## Idea

Mete el primer nodo de cada cola en un montículo de mínimos ordenado por `(turno, índice de
cola)` y recuerda `largest`, el mayor turno entre los frentes. Los frentes tienen un turno
de cada cola, así que `[mínimo, largest]` es un rango válido.

Repite: compara ese rango con el mejor hasta ahora y quédate con él solo si es
**estrictamente** más estrecho. Luego saca el nodo mínimo y mete su `next`, actualizando
`largest`. Cuando la cola sacada no tenga siguiente nodo, para.

Por qué basta: cualquier rango más estrecho `[lo, hi]` tiene, en cada cola, un turno
dentro. Mientras el mínimo del montículo esté por debajo de `lo`, avanzar la cola del
mínimo no puede saltarse el turno de esa cola dentro de `[lo, hi]`, porque es al menos
`lo`. Así el recorrido llega a un momento en que todos los frentes están en `[lo, hi]`, y
apunta un rango al menos igual de estrecho. El mínimo solo crece durante el recorrido, así
que el primer rango con el mejor ancho también tiene el menor `lo`; por eso solo los rangos
estrictamente más estrechos sustituyen al mejor.

## Complejidad

O(N log k) de tiempo para `N` clientes en `k` colas y O(k) de memoria extra para el
montículo.
