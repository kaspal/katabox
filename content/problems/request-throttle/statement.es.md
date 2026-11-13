Un pequeño servicio web se protege con un limitador: cada usuario puede tener como mucho `limit` peticiones **aceptadas** en cualquier tramo de `window` segundos. En concreto, una petición del usuario `u` en el instante `t` se acepta cuando menos de `limit` de las peticiones anteriores de `u` se aceptaron en instantes `t'` con `t − window < t' ≤ t`. Las peticiones rechazadas no cuentan.

Procesa las operaciones en orden, donde `ops[i]` es el nombre y `args[i]` los números:

- `"request"`, args `[user, time]`: **informa** `1` si la petición se acepta y `0` si se rechaza;
- `"count"`, args `[user, time]`: **informa** cuántas peticiones del usuario se aceptaron en instantes `t'` con `time − window < t' ≤ time`.

Los instantes nunca disminuyen de una operación a la siguiente. Devuelve los informes en orden.

### Ejemplo 1

```
limit = 2, window = 10, ops = ["request", "request", "request", "request", "request"], args = [[1, 1], [1, 3], [1, 5], [1, 11], [2, 11]]
Salida: [1, 1, 0, 1, 1]
```

En el instante 5, el usuario 1 ya tiene 2 peticiones aceptadas en (−5, 5]. En el instante 11, la ventana es (1, 11], que solo contiene la petición del instante 3.

### Ejemplo 2

```
limit = 1, window = 5, ops = ["request", "count", "request", "count"], args = [[7, 0], [7, 4], [7, 5], [7, 5]]
Salida: [1, 1, 1, 1]
```

### Ejemplo 3

```
limit = 3, window = 60, ops = ["count"], args = [[3, 100]]
Salida: [0]
```

### Restricciones

- 1 ≤ limit ≤ 1000, 1 ≤ window ≤ 10⁹
- 1 ≤ ops.length ≤ 10⁵, args.length = ops.length
- 1 ≤ user ≤ 10⁹, 0 ≤ time ≤ 10⁹, y los instantes nunca disminuyen
