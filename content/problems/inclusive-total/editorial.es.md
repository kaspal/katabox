## El error

La cabecera del bucle está mal:

```python
for i in range(a, b):
```

En Python, `range(a, b)` recorre `a, a + 1, ..., b - 1`: el final queda fuera. El problema pide un rango inclusivo, así que el último elemento, `nums[b]`, nunca se suma. Con `nums = [5]`, `a = 0`, `b = 0`, el bucle no da ninguna vuelta y el código devuelve `0` en lugar de `5`. En el Ejemplo 1 devuelve -2 + 7 = 5 en vez de 8.

## La corrección

Alarga el rango en uno para que incluya `b`:

```python
def range_total(nums: list[int], a: int, b: int) -> int:
    total = 0
    for i in range(a, b + 1):
        total += nums[i]
    return total
```

En TypeScript, la condición `i < b` pasa a ser `i <= b`.

**Complejidad:** O(b − a + 1) en tiempo y O(1) de espacio extra.
