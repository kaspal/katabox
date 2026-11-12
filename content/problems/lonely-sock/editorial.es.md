## Enfoque: el XOR anula las parejas

El XOR tiene tres propiedades que resuelven el problema de golpe:

- `x ^ x = 0`: un número se anula consigo mismo;
- `x ^ 0 = x`: el cero no cambia nada;
- es conmutativo y asociativo, así que el orden de los calcetines no importa.

Haz el XOR de cada número sobre un acumulador que empieza en 0. Las parejas desaparecen y el acumulador termina siendo el número solitario.

### Paso a paso

`ids = [14, 3, 9, 3, 14]`:

| número | acumulador |
| --- | --- |
| | 0 |
| 14 | 14 |
| 3 | 14 ^ 3 |
| 9 | 14 ^ 3 ^ 9 |
| 3 | 14 ^ 9 |
| 14 | 9 |

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Buscar la pareja de cada calcetín cuesta O(n²), demasiado lento para 10⁵ calcetines; un mapa de conteos es O(n) pero necesita O(n) de memoria.
