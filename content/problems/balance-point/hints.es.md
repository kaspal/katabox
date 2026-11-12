Sumar los dos lados para cada índice candidato funciona, pero cada candidato cuesta una pasada completa: O(n²) en total.
---
Si conoces el total de toda la fila y el total a la izquierda del índice `i`, ¿cuánto suma su derecha?
---
Calcula `total` una vez. Recorre de izquierda a derecha guardando `left` (la suma antes de `i`): la derecha es `total - left - weights[i]`. Devuelve el primer `i` donde coinciden; si no, suma `weights[i]` a `left` y sigue.
