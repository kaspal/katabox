## Enfoque: DP de intervalos sobre las posiciones de corte

Ordena las marcas y añade los dos extremos: `p = [0, marcas ordenadas…, length]`. Para una pieza de `p[i]` a `p[j]`, los cortes de dentro son `p[i+1..j−1]`. Sea cual sea el orden, uno de ellos, `p[k]`, se hace **primero**: cuesta la pieza entera, `p[j] − p[i]`, y deja dos piezas independientes.

```
cost[i][j] = 0                                         si j = i + 1
cost[i][j] = (p[j] − p[i]) + min_{i<k<j} (cost[i][k] + cost[k][j])   en otro caso
```

Rellena la tabla por separación `j − i` creciente; la respuesta es `cost[0][m + 1]`.

### Paso a paso

Ejemplo 1, `p = [0, 2, 4, 7, 10]`: `cost[0][2] = 4`, `cost[2][4] = 6`, … y `cost[0][4] = 10 + min(cost[0][1] + cost[1][4], cost[0][2] + cost[2][4], cost[0][3] + cost[3][4]) = 10 + min(0 + 14, 4 + 6, 11 + 0) = 20`.

**Complejidad:** O(m³) en tiempo (como mucho unos 10⁶ pasos) y O(m²) de espacio. Probar todos los órdenes de corte es factorial; probar recursivamente cada primer corte sin recordar resultados es exponencial.
