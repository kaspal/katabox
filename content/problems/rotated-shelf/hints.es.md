La estantería son dos tramos ordenados: el bloque movido al final tiene los números más pequeños. Si partes la estantería por la mitad, ¿qué puedes decir de las dos mitades?
---
Al menos una mitad está ordenada, y puedes saber cuál: `shelf[lo] ≤ shelf[m]` significa que la mitad izquierda `lo..m` está ordenada; si no, lo está la derecha `m..hi`.
---
En la mitad ordenada, una comprobación de rango te dice si `label` puede estar ahí. Si sí, busca en esa mitad; si no, en la otra. Para cuando lo encuentres o el rango quede vacío.
