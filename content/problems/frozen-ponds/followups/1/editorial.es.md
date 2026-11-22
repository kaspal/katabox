## Idea

Recorre cada estanque exactamente como en el problema base, pero cuenta lados de valla en lugar de celdas. Al visitar una celda, mira sus cuatro vecinas: una fuera del mapa o con `.` suma un lado de valla; una vecina `#` no suma nada (es del mismo estanque) y se apila si aún no se ha visto. Cada lado de cada celda helada se revisa exactamente una vez, así que el total de cada estanque es su longitud de valla. Dicho de otro modo, la valla es 4 · celdas − 2 · (parejas de sus celdas que se tocan). Ordena los totales de mayor a menor.

## Complejidad

O(R·C) en tiempo para el recorrido, más O(P log P) para ordenar los P estanques; O(R·C) de memoria extra para las marcas de visitado y la pila.
