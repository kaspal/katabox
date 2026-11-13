Sumar cada árbol de cada parcela repite mucho trabajo cuando hay muchas parcelas grandes. En una dimensión, ¿cómo responden las sumas prefijas "total de un tramo" en O(1)?
---
Sea `pre[r][c]` el total del rectángulo que va de la esquina superior izquierda hasta la fila `r − 1` y la columna `c − 1`. Cualquier parcela es una combinación de cuatro de esos rectángulos de esquina.
---
parcela = pre[bottom+1][right+1] − pre[top][right+1] − pre[bottom+1][left] + pre[top][left]. Construye `pre` fila a fila: `pre[r+1][c+1] = pre[r][c+1] + (total acumulado de la fila r hasta la columna c)`.
