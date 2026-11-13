Divide toda la entrada por espacios. Las filas del mapa no tienen espacios, así que cada fila es exactamente un token; una pregunta son dos tokens, la palabra y el índice.
---
Contar una fila o una columna en cada pregunta cuesta hasta 300 pasos, y puede haber 2·10⁴ preguntas. El mapa no cambia nunca, así que cuenta cada fila y cada columna una sola vez, antes de las preguntas.
---
Crea dos arreglos, `arbolesFila[R]` y `arbolesColumna[C]`. Por cada `#` en la fila `i`, columna `j`, suma 1 a `arbolesFila[i]` y a `arbolesColumna[j]`. Así cada pregunta es una consulta: `row r` escribe `arbolesFila[r]` y `col c` escribe `arbolesColumna[c]`.
