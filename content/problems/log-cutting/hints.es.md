Piensa en el **primer** corte que se hace dentro de una pieza: la divide en dos partes que luego se cortan por separado.
---
Ordena las marcas y añade los dos extremos: posiciones `p[0] = 0, p[1..m], p[m+1] = length`. Sea `cost[i][j]` la forma más barata de hacer todos los cortes estrictamente entre `p[i]` y `p[j]` en la pieza `p[i]..p[j]`.
---
`cost[i][j] = (p[j] − p[i]) + min para i < k < j de (cost[i][k] + cost[k][j])`, y 0 cuando `j = i + 1`. Rellénala por `j − i` creciente.
