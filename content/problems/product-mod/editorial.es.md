## El error

El código inicial empieza con `acc = 1.0`, así que todo el cálculo se hace con números de coma flotante. Tras cada paso `acc` queda por debajo de 10⁹ + 7, pero `acc * x` puede acercarse a 10¹⁸, muy por encima de 2⁵³, el límite hasta el que un double guarda todos los enteros de forma exacta. El producto se redondea al valor representable más cercano y el resto se calcula sobre ese número equivocado.

Con `nums = [999999999, 999999999]` el producto exacto es 999 999 998 000 000 001. Como double queda en 999 999 998 000 000 000 (cerca de 10¹⁸ los doubles van de 128 en 128), así que el código devuelve 63 en lugar de 64. La versión TypeScript falla igual, porque `number` también es un double.

## La corrección

Mantén el producto como entero; los enteros de Python son exactos a cualquier tamaño:

```python
def product_mod(nums: list[int]) -> int:
    acc = 1
    for x in nums:
        acc = acc * x % MOD
    return acc
```

En TypeScript, usa `BigInt` para la multiplicación (`acc = (acc * BigInt(x)) % 1000000007n`, empezando en `1n`) y devuelve `Number(acc)`, que es exacto porque el resultado es menor que 10⁹ + 7.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra.
