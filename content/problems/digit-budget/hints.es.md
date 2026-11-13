Comprobar cada boleto hasta 10¹⁵ es imposible. Construye los números cifra a cifra, empezando por la más significativa.
---
Al elegir cifras desde la izquierda, para el resto importa: la posición, la suma de cifras hasta ahora, y si el número aún coincide con el prefijo de `limit` ("ajustado") o ya es menor.
---
`count(pos, suma, ajustado)`: si está ajustado, la siguiente cifra puede llegar hasta la cifra de `limit` en `pos`; si no, hasta 9. Memoriza los estados no ajustados (se repiten mucho). Cuenta los números 0..limit con suma de cifras `total`; como total ≥ 1, el 0 nunca cuenta.
