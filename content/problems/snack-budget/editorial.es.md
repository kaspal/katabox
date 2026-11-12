## Enfoque: primero las más baratas

Si un grupo de `k` meriendas cabe en el presupuesto, las `k` más baratas también caben: no cuestan más. Así que la respuesta es el mayor `k` para el que los `k` precios más bajos suman como mucho `budget`.

```
ordenar prices
gastado = 0; cuenta = 0
para p en prices:
    si gastado + p > budget: parar
    gastado += p; cuenta += 1
devolver cuenta
```

Parar en el primer precio que no cabe es seguro: todos los siguientes son iguales o mayores.

### Paso a paso

`[120, 35, 80, 60, 200]` ordenado queda `[35, 60, 80, 120, 200]`. Sumas parciales: 35, 95, 175 y después 295 > 180, así que la respuesta es 3.

**Complejidad:** O(n log n) por la ordenación y después O(n). Buscar con un recorrido nuevo la merienda más barata que queda en cada compra cuesta O(n²).
