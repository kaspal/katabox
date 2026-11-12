## Enfoque: backtracking, coger antes que dejar, con poda

Recorre los sellos en orden de índice y decide para cada uno: cogerlo o dejarlo. Una `buscar(i, falta)` recursiva explora las elecciones para los sellos `i, i + 1, …` mientras faltan `falta` céntimos, guardando los índices elegidos en una lista:

```
buscar(i, falta):
    si falta == 0:
        si no hay mejor o len(elegidos) < len(mejor): mejor = copia de elegidos
        volver
    si i == n o falta > sufijo[i]: volver            # ya no se puede alcanzar
    si mejor y len(elegidos) + 1 >= len(mejor): volver  # no puede mejorar
    si values[i] <= falta:
        elegidos.añadir(i); buscar(i + 1, falta - values[i]); elegidos.quitar()
    buscar(i + 1, falta)
```

`sufijo[i]` es la suma de `values[i:]`.

**Por qué coger primero da el desempate correcto.** Toma dos listas de índices `A` y `B` de la misma longitud, con `A` menor en orden lexicográfico. En la primera posición donde difieren, `A` tiene un índice menor `a`. La búsqueda llega al índice `a` con las mismas elecciones anteriores para ambas listas, y explora "coger `a`" (hacia `A`) antes que "dejar `a`" (hacia `B`). Así que, entre listas igual de cortas, la primera encontrada es la menor lexicográficamente. Reemplazar la respuesta solo con **estrictamente** menos sellos la conserva.

**La poda** corta subárboles enteros: cuando ni todos los sellos restantes bastan, cuando un sello vale demasiado para cogerlo y cuando la lista actual ya es tan larga como la mejor.

### Paso a paso

`values = [5, 3, 8, 2, 6]`, objetivo 11. Coger 5 y luego 3 deja 3 por cubrir; nada encaja exacto (8 se pasa, luego 2 deja 1 y 6 se pasa), así que la búsqueda retrocede. 5 + 6 completa: mejor = [0, 4]. A partir de ahí solo podría ganar una respuesta de un sello, y ninguno vale 11, así que 3 + 8 = [1, 2] nunca se guarda.

**Complejidad:** O(2ⁿ) llamadas en el peor caso (n ≤ 18), muchas menos con la poda, y O(n) de profundidad de recursión.
