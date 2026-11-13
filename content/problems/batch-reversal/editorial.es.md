## Enfoque: invertir lote a lote y coser los extremos

Pon un nodo ficticio delante de la cinta y haz que `before` apunte a él.

1. **Comprobar el lote:** avanza k nodos por delante de `before`. Si la lista se acaba antes, para: el resto se queda como está.
2. **Invertirlo:** sea `first = before.next`. Invierte los k nodos que empiezan en `first` con los tres punteros de siempre, dejando `rest` = el nodo tras el lote.
3. **Coser:** `before.next` = el nuevo primer nodo del lote (su antiguo último), y `first.next = rest` (el antiguo primero ahora es el último).
4. **Avanzar:** `before = first`, y repite.

Lo difícil es mantener bien enlazados los dos extremos; el nodo ficticio elimina el caso especial del primer lote.

### Paso a paso

`[1..8]`, `k = 3`: el lote 1, 2, 3 pasa a 3, 2, 1 y el 1 enlaza con el 4; el lote 4, 5, 6 pasa a 6, 5, 4 y el 4 enlaza con el 7; después solo quedan 7 y 8, menos de 3, así que se quedan igual.

**Complejidad:** O(n) en tiempo (cada nodo se visita dos veces) y O(1) de espacio extra.
