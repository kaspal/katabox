Mira las últimas notas de las dos melodías. O bien coinciden y pueden cerrar la melodía compartida, o bien alguna de ellas no forma parte.
---
Sea `L[i][j]` la respuesta para las `i` primeras notas de `first` y las `j` primeras de `second`.
---
Si `first[i − 1] == second[j − 1]`, `L[i][j] = L[i − 1][j − 1] + 1`; si no, `L[i][j] = max(L[i − 1][j], L[i][j − 1])`. Rellena la tabla fila a fila; basta con guardar dos filas.
