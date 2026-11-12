Piensa en cada celda helada como un nodo, unido a las celdas heladas de su derecha y de abajo. Un estanque es un grupo de nodos conectados.
---
Una estructura de conjuntos disjuntos (union-find) guarda un representante por grupo: empieza con cada celda sola y une cada celda helada con sus vecinas heladas de la derecha y de abajo.
---
Numera la celda `(r, c)` como `r × ancho + c`. Con compresión de caminos y unión por tamaño, cada unión cuesta casi O(1). Al final, cuenta las celdas de cada representante y ordena los recuentos de mayor a menor.
