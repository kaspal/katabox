Comparar cada nodo solo con sus dos hijos no basta (mira el ejemplo 2): la regla implica a todos los antecesores.
---
Cada nodo debe quedar estrictamente entre unos límites que fijan sus antecesores: ir a la izquierda baja el límite superior al valor del padre, ir a la derecha sube el inferior.
---
Recurre con `(nodo, bajo, alto)`, empezando sin límites. Falla cuando `nodo.val ≤ bajo` o `nodo.val ≥ alto`. Otra opción: un recorrido en orden de un árbol de búsqueda visita valores estrictamente crecientes.
