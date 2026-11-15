Simula la impresora con un reloj `t`. La entrada no está en orden de llegada, así que primero ordena los trabajos por llegada (guardando el número de cada uno). Así, los trabajos que han llegado hasta el instante `t` son siempre un prefijo de esa lista ordenada.
---
Recorrer todos los trabajos en espera en cada elección es correcto pero lento: en una oficina con mucho trabajo esperan decenas de miles a la vez. Guarda los trabajos en espera en una **cola de prioridad** (montículo) ordenada por la regla de la impresora, para que cada elección cueste O(log n).
---
Ordena el montículo por la clave `(−priority, arrival, número)`, de menor a mayor. Repite hasta imprimir todos los trabajos: si el montículo está vacío y el siguiente trabajo llega después de `t`, salta `t` a su llegada; mete todos los trabajos con `arrival ≤ t`; saca el de arriba, anota su número y suma sus páginas a `t`.
