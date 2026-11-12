Un valle tiene `n` aldeas, numeradas del `0` al `n − 1`, y quiere una red de fibra que permita a cada aldea llegar a todas las demás, directamente o pasando por otras aldeas. Los topógrafos apuntaron los cables que se podrían tender: `cables[i] = [a, b, cost]` es un cable entre las aldeas `a` y `b` que cuesta `cost` monedas.

Devuelve el **menor coste total** de un conjunto de cables que conecte todas las aldeas, o `-1` si incluso tendiendo todos los cables apuntados alguna aldea queda aislada.

### Ejemplo 1

```
n = 4, cables = [[0, 1, 7], [1, 2, 3], [0, 2, 5], [2, 3, 4], [1, 3, 9]]
Salida: 12
```

Se tienden 1–2 (3), 2–3 (4) y 0–2 (5). Todas las aldeas quedan conectadas por 12 monedas.

### Ejemplo 2

```
n = 3, cables = [[0, 1, 2]]
Salida: -1
```

La aldea 2 no tiene ningún cable.

### Ejemplo 3

```
n = 1, cables = []
Salida: 0
```

### Restricciones

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ cables.length ≤ 4·10⁴
- `cables[i] = [a, b, cost]` con 0 ≤ a, b < n, a ≠ b y 1 ≤ cost ≤ 10⁶; dos cables pueden unir las mismas aldeas
