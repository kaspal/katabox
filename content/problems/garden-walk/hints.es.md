Todo paseo tiene una fuente más alta, donde pasa de subir a bajar. ¿Qué paseos giran en un nodo dado?
---
Un paseo que gira en el nodo v baja por su subárbol izquierdo lo más hondo posible y por su subárbol derecho lo más hondo posible.
---
Calcula la altura de cada subárbol (en caminos) de abajo arriba. En el nodo v, un paseo que gira ahí tiene `altura(izq) + altura(der) + 2` caminos cuando existen los dos hijos (cuenta un hijo que falta como altura −1). Quédate con el máximo sobre todos los nodos.
