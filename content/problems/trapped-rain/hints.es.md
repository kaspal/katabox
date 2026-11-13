Para una columna, el agua que tiene encima es `min(la más alta a la izquierda, la más alta a la derecha) − altura`, si es positivo. Recorrer los dos lados para cada columna es O(n²).
---
Los valores de "la más alta a la izquierda" son un máximo acumulado desde la izquierda, y los de "la más alta a la derecha", un máximo acumulado desde la derecha: dos arrays construidos en O(n).
---
Para usar espacio O(1), mueve dos punteros hacia dentro con los máximos acumulados `leftMax` y `rightMax`. Si `leftMax ≤ rightMax`, el agua de la columna izquierda la decide `leftMax` (a la derecha espera algo al menos igual de alto): suma `leftMax − altura` y avanza la izquierda. Si no, haz lo mismo por la derecha.
