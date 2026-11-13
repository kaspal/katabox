Hacer crecer un tramo desde cada emisión de partida repite casi todo el trabajo cuando los tramos son largos. ¿Qué le pasa a un tramo fresco al añadir la siguiente emisión?
---
Mantén una ventana `[start, end]` que siempre sea fresca. Cuando la nueva emisión `x` ya sonó dentro de la ventana, la ventana debe empezar justo después de esa emisión anterior de `x`.
---
Recuerda la última posición de cada canción (un mapa hash). Para cada `end`: si `last[x] ≥ start`, pon `start = last[x] + 1`; después `last[x] = end`, y la longitud de la ventana es `end − start + 1`.
