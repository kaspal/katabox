## Enfoque: construir cada entrada a partir de otra menor

`i >> 1` es `i` sin su último dígito binario, e `i & 1` es ese último dígito. Así que

`bits[0] = 0` y `bits[i] = bits[i >> 1] + (i & 1)`.

Como `i >> 1 < i`, la entrada necesaria siempre está ya en la tabla.

### Paso a paso

bits[6]: 6 = 110, 6 >> 1 = 3 = 11 con 2 bits, y 6 & 1 = 0, así que 2. bits[7] = bits[3] + 1 = 3.

**Complejidad:** O(n) en tiempo, un paso por entrada. Contar los bits de cada número uno a uno es O(n log n), también válido aquí; la versión con tabla muestra cómo una identidad a nivel de bits se convierte en un DP.
