Revisar todas las rachas es O(n²). Lleva un marcador acumulado: +1 por victoria, −1 por
derrota, 0 por empate. ¿Cómo se ve una racha equilibrada en ese marcador?
---
Una racha es equilibrada justo cuando el marcador tras su último partido es igual al de
justo antes del primero. Para la racha más larga que termina en el partido `i`, ¿qué
momento anterior con el mismo marcador te interesa?
---
Recuerda la **primera** posición en que apareció cada marcador, empezando con el
marcador 0 en la posición −1. En cada partido, si el marcador actual ya salió antes, la
racha desde esa primera vez es equilibrada; quédate con la más larga. Nunca sobrescribas
una primera posición.
