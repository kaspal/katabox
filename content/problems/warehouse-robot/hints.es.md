Piensa en cada celda libre como un nodo conectado con sus vecinas libres. Necesitas el menor número de aristas desde el nodo de salida hasta el de la meta.
---
Cuando todos los movimientos cuestan lo mismo, explorar las celdas por orden de distancia encuentra la ruta más corta. ¿Qué recorrido hace eso?
---
Búsqueda en anchura: una cola que empieza en la celda superior izquierda con distancia 0. Saca una celda y mete cada vecina libre y no visitada con distancia + 1, marcándola como visitada al meterla. Para cuando saques la meta.
