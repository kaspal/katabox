Las dos operaciones afectan a un rango entero. Una estructura que divide las estaciones en segmentos anidados, cada uno con su propio mínimo, puede responder un rango con O(log n) segmentos.
---
Es un árbol de segmentos. Un `"shift"` de un segmento entero cambia también su mínimo en `delta`, así que puedes anotar el desplazamiento en el segmento en lugar de visitar cada estación.
---
Guarda en cada nodo `min` (de su segmento, con su desplazamiento pendiente incluido) y `pending` (un desplazamiento aún no pasado a sus hijos). Baja el desplazamiento pendiente antes de visitar los hijos; tras actualizarlos, `min = min(izq.min, der.min)`.
