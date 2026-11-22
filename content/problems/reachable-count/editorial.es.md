## El error

La búsqueda en anchura marca una sala como vista al meterla en la cola, lo cual es correcto, pero la sala inicial entra en la cola sin marcarse nunca:

```python
    seen = [False] * n
    queue = deque([start])
```

Cuando se procesa un vecino de `start`, `seen[start]` sigue en `False`, así que `start` vuelve a la cola y se cuenta otra vez. Cualquier sala inicial con al menos una puerta se cuenta dos veces.

Con `n = 2, doors = [[0, 1]], start = 0`: se saca la sala 0 (cuenta 1) y encola la 1; se saca la 1 (cuenta 2) y vuelve a encolar la 0; se saca la 0 (cuenta 3). El código devuelve 3, pero solo hay 2 salas.

## La corrección

Marca la sala inicial antes de empezar la búsqueda:

```python
    seen = [False] * n
    seen[start] = True
    queue = deque([start])
```

El código TypeScript devuelve `queue.length`, así que el duplicado también aparece ahí; añade `seen[start] = true;` antes del bucle.

**Complejidad:** O(n + d) en tiempo y O(n + d) de espacio extra, donde d es el número de puertas.
