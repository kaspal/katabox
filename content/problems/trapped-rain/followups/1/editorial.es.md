## Idea

Las columnas del borde forman el "muro" inicial alrededor del interior desconocido. Guarda ese muro en un montículo de mínimos ordenado por nivel del agua. La entrada más baja del muro, de nivel `L`, resuelve a sus vecinas sin visitar: cualquier salida del interior cruza el muro a un nivel ≥ `L`, y esta vecina puede salir por la entrada de nivel `L`, así que su nivel del agua es `max(L, h)` para su altura `h`. Guarda `max(0, L − h)` de agua y entra en el muro con nivel `max(L, h)`. Repetir desde el punto más bajo del muro (el algoritmo de Dijkstra con "columna más alta del camino" en lugar de "longitud del camino") visita cada columna una vez y calcula exactamente la definición. Los dos punteros del problema base son la versión de una sola fila de la misma idea: mover siempre el lado más bajo.

## Complejidad

O(R·C·log(R·C)) en tiempo por las operaciones del montículo y O(R·C) de memoria extra. El total cabe de sobra en 32 bits: como mucho 4·10⁴ columnas × 10⁴ = 4·10⁸.
