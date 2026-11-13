## Enfoque: una pasada en tres fases

Recorre las franjas ordenadas con un índice `i`:

1. **Izquierda:** mientras `slots[i].end < fresh.start`, la franja está entera antes de la nueva: cópiala.
2. **Solape:** mientras `slots[i].start ≤ fresh.end`, la franja comparte un momento con la nueva (que va creciendo): pon `fresh = [mín(inicios), máx(fines)]` y sáltala.
3. Añade `fresh` y copia las franjas restantes: empiezan después de que termine `fresh`.

Como las franjas antiguas estaban ordenadas y separadas, el resultado también lo está.

### Paso a paso

Ejemplo 1: [1, 3] termina antes del 7, así que se copia. [6, 8] empieza en 6 ≤ 10: fresh pasa a [6, 10]. [10, 12] empieza en 10 ≤ 10: fresh pasa a [6, 12]. No queda nada más.

**Complejidad:** O(n) en tiempo.
