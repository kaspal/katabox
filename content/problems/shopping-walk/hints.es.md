Hacer crecer un tramo desde cada puesto de partida hasta que cubra la lista puede ser O(n²). Cuando un tramo ya cubre la lista, ¿qué puedes hacer con su extremo izquierdo?
---
Usa una ventana: extiende el extremo derecho hasta que la ventana cubra la lista, y después avanza el izquierdo mientras siga cubriéndola. Cada ventana que cubre tiene ocasión de ser la más corta.
---
Lleva `missing`, el número de artículos de la lista que la ventana aún no cubre. Añadir un puesto de una letra que aún falta lo reduce; quitar uno que deja una letra por debajo de lo necesario lo aumenta. Anota la longitud siempre que `missing = 0`.
