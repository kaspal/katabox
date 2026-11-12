Mira las últimas letras de las dos palabras. Si son iguales, ¿necesitan algún arreglo? Si no, ¿cuáles son las tres maneras de tratar la última letra?
---
Sea `d[i][j]` los arreglos necesarios para convertir las primeras `i` letras de `typed` en las primeras `j` letras de `meant`. La recursión simple recalcula los mismos pares `(i, j)` un número exponencial de veces.
---
`d[i][0] = i`, `d[0][j] = j`. Si `typed[i-1] == meant[j-1]`, `d[i][j] = d[i-1][j-1]`; si no, `d[i][j] = 1 + min(d[i-1][j]` (borrar), `d[i][j-1]` (insertar), `d[i-1][j-1]` (sustituir)`)`. Rellena la tabla fila a fila; con dos filas basta.
