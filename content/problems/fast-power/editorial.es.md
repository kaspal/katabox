## Enfoque: exponenciación por cuadrados

Escribe el exponente en binario. Lleva `cuadrado = base mod p` (irá guardando base¹, base², base⁴, base⁸, …) y `resultado = 1`. Mientras `exponente > 0`:

- si su bit más bajo es 1, `resultado = resultado · cuadrado mod p`;
- `cuadrado = cuadrado · cuadrado mod p`;
- `exponente = ⌊exponente / 2⌋`.

Cada multiplicación va seguida de `mod p`, así que los dos factores están por debajo de p = 1 000 003 y el producto por debajo de unos 10¹², exacto en todos los lenguajes de aquí (incluidos los números de JavaScript, exactos hasta unos 9·10¹⁵).

### Paso a paso

3¹³ con 13 = 1101 en binario: el resultado recoge 3 (bit 1), salta 3² (bit 0), recoge 3⁴ = 81 y 3⁸ = 6561: 3 · 81 · 6561 = 1 594 323 → 594 320.

**Complejidad:** O(log exponente) multiplicaciones, unas 50 para 10¹⁵. Multiplicar `exponent` veces es O(exponent).
