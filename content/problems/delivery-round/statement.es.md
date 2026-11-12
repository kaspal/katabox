Una mensajera en bici sale del almacén (la parada `0`), entrega en cada una de las demás paradas exactamente una vez, en cualquier orden, y vuelve al almacén. Ir de la parada `i` directamente a la parada `j` lleva `dist[i][j]` minutos. La ciudad tiene calles de sentido único, así que `dist[i][j]` y `dist[j][i]` pueden ser distintos, y un rodeo por otra parada nunca forma parte de un trayecto: cada trayecto va directo a la siguiente parada.

Devuelve los minutos mínimos que puede llevar toda la ruta. Si solo está el almacén, la ruta lleva 0 minutos.

### Ejemplo 1

```
dist = [[0, 2, 9], [1, 0, 6], [15, 7, 0]]
Salida: 17
```

La ruta 0 → 1 → 2 → 0 lleva 2 + 6 + 15 = 23 minutos, pero 0 → 2 → 1 → 0 lleva 9 + 7 + 1 = 17.

### Ejemplo 2

```
dist = [[0]]
Salida: 0
```

### Ejemplo 3

```
dist = [[0, 5], [3, 0]]
Salida: 8
```

### Restricciones

- 1 ≤ n ≤ 12, con `n = dist.length` y cada `dist[i].length = n`
- dist[i][i] = 0, y 1 ≤ dist[i][j] ≤ 10⁴ para i ≠ j
