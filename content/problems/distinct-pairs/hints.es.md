Las listas sin precios repetidos salen bien. Prueba una en la que un precio que encaja aparece dos veces a cada lado, como `prices = [1, 1, 3, 3], budget = 4`.
---
Mover los punteros cuando la suma se queda corta o se pasa está bien. Fíjate en lo que ocurre justo después de contar una coincidencia.
---
Tras una coincidencia, el código mueve cada puntero una sola posición, así que el siguiente paso puede caer en los mismos dos valores y contarlos otra vez. En su lugar, avanza `i` más allá de todas las copias de `prices[i]` y retrocede `j` más allá de todas las copias de `prices[j]` (manteniendo `i < j`) antes de seguir.
