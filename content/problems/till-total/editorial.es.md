## El error

El código inicial suma los precios como números en coma flotante y convierte al final:

```python
total = 0.0
for p in prices:
    total += float(p)
return int(total * 100)
```

La coma flotante binaria no puede guardar exactamente la mayoría de los valores con dos decimales. `4.35` se guarda en realidad como 4.34999999999999964..., así que `total * 100` queda un pelo por debajo del número real de céntimos, e `int` corta la parte decimal en lugar de redondear. La suma también puede ir acumulando pequeños errores.

Con `prices = ["4.35"]`, `float("4.35") * 100` vale `434.99999999999994`, así que el código devuelve `434` en lugar de `435`.

## La corrección

No salgas nunca de los enteros. Cada precio tiene exactamente dos decimales, así que al quitar el punto obtienes su valor en céntimos, y la suma de enteros es exacta:

```python
return sum(int(p.replace(".", "")) for p in prices)
```

En TypeScript, suma `Number(p.replace(".", ""))` para cada precio y devuelve el total; el mayor total posible, unos 10¹⁰ céntimos, sigue siendo un entero exacto en un número de JavaScript. (Redondear con `Math.round(total * 100)` parchea estas entradas, pero contar en céntimos es exacto por construcción).

**Complejidad:** O(L) en tiempo, donde L es la longitud total de las cadenas, y O(1) de espacio extra.
