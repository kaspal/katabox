## El error

El código inicial calcula cada nivel con una función auxiliar recursiva con memoria:

```python
def level(v: int) -> int:
    if manager[v] == -1:
        return 1
    if depth[v] == 0:
        depth[v] = level(manager[v]) + 1
    return depth[v]
```

La lógica es correcta, y la memoria hace que el trabajo total sea lineal. El problema es la profundidad de la recursión: cuando `level` empieza por una persona cuyos superiores aún no se conocen, hace una llamada anidada por cada responsable en el camino hacia arriba. Las restricciones permiten una única cadena de 5·10⁴ personas, y tantas llamadas anidadas desbordan la pila, así que la ejecución termina con un error en tiempo de ejecución en lugar de devolver una respuesta.

El orden importa. Con `manager = [-1, 0, 1, ...]` (cada persona reporta a la anterior), el bucle visita primero a la jefa y cada llamada posterior se detiene tras un paso en una profundidad conocida. Con la cadena al revés, `manager = [1, 2, 3, ..., 49999, -1]` (5·10⁴ personas, cada una reporta a la siguiente y la última es la jefa), la primera llamada `level(0)` tiene que bajar 5·10⁴ llamadas antes de memorizar nada. La respuesta es 50000, pero el código falla.

Subir el límite de recursión no es una solución fiable: la pila real puede seguir agotándose, y con organigramas más profundos el fallo solo se desplazaría.

## La corrección

Sustituye la recursión por un recorrido explícito. Sube desde `v` hasta llegar a la jefa o a alguien con profundidad conocida, guardando el camino, y luego rellena las profundidades bajando por ese camino:

```python
def deepest_report(manager: list[int]) -> int:
    n = len(manager)
    depth = [0] * n
    best = 0
    for v in range(n):
        path = []
        u = v
        while u != -1 and depth[u] == 0:
            path.append(u)
            u = manager[u]
        d = 0 if u == -1 else depth[u]
        for w in reversed(path):
            d += 1
            depth[w] = d
        best = max(best, depth[v])
    return best
```

La versión en TypeScript tiene la misma función recursiva `level`; sustitúyela por el mismo bucle de subir y rellenar, usando un array normal para el camino.

**Complejidad:** O(n) en tiempo, porque cada persona entra en un camino una sola vez, y O(n) de espacio extra para `depth` y el camino.
