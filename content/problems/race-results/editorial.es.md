## El error

El código ordena dos veces y se apoya en que la ordenación de Python es estable: primero por nombre y luego por tiempo, de modo que los corredores con el mismo tiempo conservan el orden de nombres de la primera pasada. El problema está en esa primera pasada:

```python
order = sorted(range(len(names)), key=lambda i: names[i], reverse=True)
```

`reverse=True` deja los nombres en orden **descendente**, así que todos los empates acaban en orden alfabético inverso. Las entradas con todos los tiempos distintos no se ven afectadas, y por eso el fallo pasa desapercibido. Con `names = ["ana", "bo"]` y `times = [5, 5]`, el código devuelve `["bo", "ana"]` en lugar de `["ana", "bo"]`. El Ejemplo 2 devuelve `["kim", "joe", "eli", "ben"]`.

## La corrección

Ordena los nombres de forma ascendente antes de la ordenación estable por tiempo:

```python
def race_order(names: list[str], times: list[int]) -> list[str]:
    order = sorted(range(len(names)), key=lambda i: names[i])
    order.sort(key=lambda i: times[i])
    return [names[i] for i in order]
```

También sirve una sola ordenación con la clave `(times[i], names[i])`. En TypeScript, invierte el desempate para que quede `names[a] < names[b] ? -1 : 1`.

**Complejidad:** O(n log n) en tiempo (por la longitud de los nombres en cada comparación) y O(n) de espacio extra.
