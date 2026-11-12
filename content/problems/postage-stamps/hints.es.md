Con 18 sellos como mucho hay como mucho 2¹⁸ ≈ 262 000 elecciones, así que explorarlas es viable si cada paso es barato. Decide para el sello 0, luego para el 1, y así: cogerlo o dejarlo.
---
Busca recursivamente con el índice del siguiente sello y la cantidad que falta. Prueba a **coger** un sello antes de **dejarlo**: así las listas con el mismo número de sellos aparecen en orden lexicográfico, y basta con guardar una respuesta nueva cuando usa estrictamente menos sellos.
---
Poda: para cuando falte 0 (guarda la respuesta), cuando los sellos que quedan no lleguen a sumarlo, cuando el siguiente sello valga más de lo que falta (no lo cojas) y cuando ya uses tantos sellos como la mejor respuesta.
