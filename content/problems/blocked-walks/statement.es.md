Las calles de una ciudad forman una cuadrícula. Un repartidor sale del cruce `(0, 0)` y debe llegar al cruce `(rows, cols)`; cada paso va al siguiente cruce hacia el sur (sumando 1 al primer número) o hacia el este (sumando 1 al segundo). Algunos cruces están cortados por obras: `blocked[i] = [r, c]`, y un paseo no puede pasar por ellos.

Devuelve el número de paseos distintos. La respuesta puede ser enorme, así que devuélvela **módulo 1 000 003** (un primo): el resto de dividirla entre 1 000 003.

### Ejemplo 1

```
rows = 2, cols = 2, blocked = [[1, 1]]
Salida: 2
```

De los 6 paseos, 4 pasan por el cruce del centro.

### Ejemplo 2

```
rows = 3, cols = 1, blocked = []
Salida: 4
```

### Ejemplo 3

```
rows = 2, cols = 3, blocked = [[0, 1], [1, 0]]
Salida: 0
```

Los dos primeros pasos están cortados.

### Ejemplo 4

```
rows = 100000, cols = 100000, blocked = []
Salida: 306056
```

### Restricciones

- 1 ≤ rows, cols ≤ 10⁵
- 0 ≤ blocked.length ≤ 1000
- 0 ≤ r ≤ rows, 0 ≤ c ≤ cols; los cruces cortados son todos distintos y no incluyen ni `(0, 0)` ni `(rows, cols)`.
