## Enfoque: simulación por eventos con un montículo de trabajos en espera

**Primero, ordenar las llegadas.** Los trabajos vienen en el orden de la entrada, no en orden de llegada, así que construye `(arrival, priority, pages, número)` para cada trabajo y ordénalos por llegada (y después por número). Con un puntero `next` en esa lista, los trabajos que han llegado hasta el instante `t` son exactamente los anteriores a `next`, una vez que lo has avanzado más allá de cada `arrival ≤ t`.

**La sala de espera es un montículo.** La impresora siempre quiere la clave más pequeña

```
(−priority, arrival, número)
```

entre los trabajos en espera, que es justo lo que da un montículo de mínimos. La simulación:

```
t = 0
mientras queden trabajos:
    si el montículo está vacío y jobs[next].arrival > t: t = jobs[next].arrival   # parada
    mientras next < n y jobs[next].arrival ≤ t: mete jobs[next]; next += 1
    saca la clave más pequeña; escribe su número; t += sus páginas
```

Ejemplo 1: en `t = 0` solo el trabajo 1 está en el montículo; se imprime hasta `t = 3`. Entran los trabajos 2, 3 y 4 (llegadas 1, 1 y 2). La clave más pequeña es la del trabajo 3, `(−9, 1, 3)`; `t = 4`. Después `(−5, 1, 2)` gana a `(−5, 2, 4)`, así que se imprime el 2 y luego el 4: `1 3 2 4`.

**Por qué es correcto:** la impresora solo elige en los momentos en que queda libre, y el bucle se ejecuta justo en esos momentos: al terminar un trabajo o, si no hay nada esperando, en la siguiente llegada. En cada uno de esos `t`, el montículo contiene exactamente los trabajos con `arrival ≤ t` que no se han impreso (todos entran antes de sacar, y los impresos ya salieron). Sacar la clave más pequeña aplica la regla completa: primero la mayor prioridad, después la llegada más temprana y después el menor número. Los trabajos no se interrumpen entre sí, así que no pasa nada entre una elección y la siguiente.

**Complejidad:** O(n log n) para ordenar, y cada trabajo entra y sale una vez del montículo con coste O(log n): O(n log n) en total.

**Por qué recorrer la lista es demasiado lento:** guardar los trabajos en espera en una lista normal y recorrerla para encontrar el mejor cuesta O(en espera) por elección. En una oficina con mucho trabajo (tres trabajos por segundo, de unas cinco páginas cada uno), la cola crece hasta decenas de miles de trabajos; en 5·10⁴ elecciones son unas 10⁹ comparaciones, muy por encima del límite de tiempo, mientras que el montículo necesita unos 5·10⁴ · 2 · 16 ≈ 1.6·10⁶ pasos.

**Notas prácticas:**

- El reloj puede llegar a 10⁹ + 5·10⁴ · 10⁴ = 1.5·10⁹: usa enteros de 64 bits en Rust (`i64`/`u64`). Los enteros de PHP y el `Number` de TypeScript sirven.
- Python: `heapq` con tuplas `(-p, a, i, g)` compara justo en el orden de la impresora. Rust: `BinaryHeap` es un montículo de máximos, así que mete `Reverse((-p, a, i, g))` o `(p, Reverse(a), Reverse(i))`. TypeScript no tiene montículo en su biblioteca estándar: escribe un pequeño montículo binario con un comparador. PHP: `SplPriorityQueue` no conserva el orden de inserción entre prioridades iguales, así que pasa la clave completa como prioridad, `[$p, -$a, -$i]` (es una cola de máximos y un arreglo se compara elemento a elemento), o escribe un pequeño montículo.
- Lee toda la entrada estándar de una vez y escribe los `n` números de una sola vez.
