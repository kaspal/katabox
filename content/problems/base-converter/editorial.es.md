## Enfoque: leer a un entero y volver a escribirlo

**Leer:** el valor de cada dígito es su posición en `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ`. De izquierda a derecha, `valor = valor · source + dígito` (regla de Horner): "1011" en base 2 es ((1·2 + 0)·2 + 1)·2 + 1 = 11.

**Escribir:** mientras `valor > 0`, el siguiente dígito por la derecha es `valor mod target`, y después `valor = ⌊valor / target⌋`. Da la vuelta a los dígitos obtenidos. Si el valor es 0, la respuesta es `"0"`.

El valor es como mucho 10¹⁵, que todos los lenguajes de aquí guardan exacto en un número normal (el límite de JavaScript para enteros exactos es de unos 9·10¹⁵).

### Paso a paso

"Z" en base 36 es 35. En base 3: 35 mod 3 = 2 (35 → 11), 11 mod 3 = 2 (→ 3), 3 mod 3 = 0 (→ 1), 1 mod 3 = 1 (→ 0). Al revés: "1022".

**Complejidad:** O(dígitos) en tiempo.
