## El error

La idea es buena: guardar como mucho `k` precios en un montículo y, cuando haya `k + 1`, descartar el que no puede estar entre los `k` más baratos, que es el más caro. Pero `heapq` es un montículo de mínimos, así que estas líneas descartan el más barato:

```python
        heapq.heappush(heap, p)
        if len(heap) > k:
            heapq.heappop(heap)
```

El montículo acaba con los `k` precios más grandes y el código devuelve su suma. Solo acierta cuando `k` es igual a la longitud de la lista, porque entonces nunca se saca nada.

Con `prices = [1, 5], k = 1`: tras meter 5 el montículo tiene 1 y 5, `heappop` quita el 1 y el código devuelve 5. La respuesta es 1.

## La corrección

Conviértelo en un montículo de máximos guardando los precios negados, de modo que la raíz sea el precio más caro que se conserva:

```python
    for p in prices:
        heapq.heappush(heap, -p)
        if len(heap) > k:
            heapq.heappop(heap)
    return -sum(heap)
```

En TypeScript, la clase `Heap` recibe el orden como argumento, así que créala con `new Heap((a, b) => a > b)`.

**Complejidad:** O(n log k) en tiempo y O(k) de espacio extra.
