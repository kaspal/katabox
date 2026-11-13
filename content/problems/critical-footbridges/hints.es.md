Cerrar cada pasarela por turnos y comprobar la conectividad es O(m·(n + m)). Una pasarela es crítica exactamente cuando no está en ningún circuito. ¿Cómo puede un solo DFS encontrar los circuitos?
---
Numera las islas en orden de DFS (`tin`). Para cada isla, calcula `low`: el menor `tin` al que se llega desde su subárbol del DFS usando como mucho una pasarela que vuelve hacia arriba en el árbol.
---
La pasarela del árbol del padre p al hijo c es crítica exactamente cuando `low[c] > tin[p]`: nada del subárbol de c puede volver a subir hasta p o más arriba. Salta solo la pasarela por la que bajaste (no todas las que van al padre). Usa una pila explícita para las cadenas largas.
