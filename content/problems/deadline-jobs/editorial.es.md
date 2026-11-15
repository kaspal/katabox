## Enfoque: primero los mejores honorarios, en el día libre más tardío (union-find sobre días)

**El voraz.** Ordena los encargos por honorarios, de mayor a menor. Da a cada uno el día libre más tardío hasta su plazo; si todos los días hasta su plazo están ocupados, recházalo. Los plazos mayores que `n` se recortan a `n`: se hacen como mucho `n` encargos, así que cualquier plan cabe en los días `1..n`.

Ejemplo 1, en orden de honorarios: 50 (plazo día 2) ocupa el día 2; 40 (plazo día 2) ocupa el día 1; 30 (plazo día 1) encuentra ocupados los días 1 y 2 y se rechaza; 20 (plazo día 3) ocupa el día 3; 10 se rechaza. Total 110.

**Por qué es correcto.** Usa este criterio: un conjunto de encargos se puede hacer entero exactamente cuando, para cada día `d`, como mucho `d` de ellos vencen el día `d` o antes (entonces hacerlos en orden de plazo cumple todos los plazos).

- *La búsqueda del día responde "¿todavía cabe este encargo?"* Si todos los días hasta el plazo del encargo están ocupados, sea `m` el final del tramo de días ocupados que empieza en el día 1. Todo encargo de los días `1..m` vence el día `m` o antes: uno que venciera más tarde se habría puesto en el día libre `m + 1` o después. Así que los `m` encargos ocupados y el nuevo vencen todos el día `m` o antes, y el criterio falla. Si no, hay un día libre y el encargo cabe.
- *Quedarse un encargo que cabe nunca perjudica.* Supón que los encargos aceptados hasta ahora, `G`, forman parte de algún plan óptimo `B`, y que el siguiente encargo `j` cabe con `G` pero no está en `B`. Si `B + j` cumple el criterio, paga más que `B`: imposible. Si no, toma el primer día `d` en que `B + j` tiene `d + 1` encargos con plazo hasta `d`. `G + j` cumple el criterio, así que algún encargo `k` con plazo hasta `d` está en `B` pero no en `G`. Los encargos anteriores en orden de honorarios que se rechazaron no cabían con parte de `G`, así que tampoco están en `B`: `k` viene después en orden de honorarios, y paga como mucho lo que `j`. Cambiar `k` por `j` arregla todos los días sobrecargados (todos son `d` o posteriores, y `k` vence como tarde en `d`) y no paga menos. Así que algún plan óptimo contiene `G + j`.
- *Rechazar un encargo que no cabe es obligado:* ningún plan que contenga `G` puede incluirlo.

Por inducción, los encargos aceptados forman un plan óptimo.

**Encontrar el día rápido.** Buscar hacia atrás día a día es la parte lenta: con 10⁵ encargos que vencen todos el día 5·10⁴ o antes, la mitad recorren casi todos los días, unos 3.7·10⁹ pasos. En su lugar guarda `free[d]`, que apunta hacia el día libre más tardío ≤ `d`, donde el día 0 significa "no queda ningún día libre". Empieza con `free[d] = d`. Para colocar un encargo, sigue los punteros desde `min(deadline, n)` hasta un día que se apunte a sí mismo, acortando el camino a la mitad sobre la marcha (`free[d] = free[free[d]]`). Ocupar el día `d` pone `free[d] = d − 1`, así que las búsquedas siguientes lo saltan directamente.

**Complejidad:** O(n log n) por la ordenación, más casi O(1) por encargo en el union-find: tiempo O(n log n), memoria O(n).

**Notas prácticas:** lee todos los tokens de una vez. El total puede llegar a 10⁵ · 10⁹ = 10¹⁴: usa `u64` o `i64` en Rust; en TypeScript queda por debajo de 2⁵³, así que `Number` es exacto, y los enteros de PHP son de 64 bits. Escribe la búsqueda como un bucle, no con recursión (una cadena de 10⁵ días desbordaría el límite de recursión de Python). Los empates de honorarios se pueden tomar en cualquier orden.
