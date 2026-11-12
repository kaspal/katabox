Revisar cada racha son unas n²/2 rachas. En su lugar, lleva un marcador acumulado: +1 por victoria, −1 por derrota y 0 por empate. ¿Cómo es el marcador en los dos extremos de una racha equilibrada?
---
Una racha del partido `i + 1` al partido `j` está equilibrada exactamente cuando el marcador acumulado tras el partido `j` es igual al marcador tras el partido `i` (el marcador antes del primer partido es 0).
---
Así que cuenta parejas de marcadores iguales. Recorre la temporada con un mapa hash de cuántas veces ha salido cada marcador; cada posición nueva suma el número de posiciones anteriores con el mismo marcador.
