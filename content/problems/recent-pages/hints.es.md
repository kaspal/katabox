Guardar las páginas en una lista ordenada por último uso funciona, pero encontrar y mover una página cuesta O(capacity). ¿Qué dos cosas tienen que ser rápidas?
---
Encontrar una página por número (un mapa hash), y mover una página al extremo "más reciente" o quitar la "menos reciente" (una lista doblemente enlazada hace las dos en O(1)).
---
Asocia cada página a su nodo en una lista doblemente enlazada ordenada por uso. "get" y "put" mueven el nodo al frente; desalojar quita el nodo del final y su entrada del mapa. Muchos lenguajes tienen un mapa ordenado que lo hace por ti (mapas con orden de inserción en los que borras y vuelves a insertar para refrescar).
