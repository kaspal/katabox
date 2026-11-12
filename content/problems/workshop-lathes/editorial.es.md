## Enfoque: un montículo de mínimos con los instantes libres

La única pregunta del encargado es "¿qué torno queda libre antes?", y la respuesta cambia tras cada trabajo. Un montículo de mínimos la responde en O(log m):

1. Mete `min(lathes, n)` ceros: los tornos que sobran respecto al número de trabajos nunca importan.
2. Para cada trabajo en orden: saca el menor instante libre `t`, el trabajo termina en `t + job` y vuelve a meterlo.
3. La respuesta es el mayor instante de fin.

Como todos los tornos son iguales, *qué* torno recibe un trabajo cuando varios están libres a la vez no cambia ningún instante.

### Paso a paso

`jobs = [5, 3, 8, 2, 4]`, `lathes = 2`, montículo `{0, 0}`:

| trabajo | saca | mete | montículo | último fin |
| --- | --- | --- | --- | --- |
| 5 | 0 | 5 | {0, 5} | 5 |
| 3 | 0 | 3 | {3, 5} | 5 |
| 8 | 3 | 11 | {5, 11} | 11 |
| 2 | 5 | 7 | {7, 11} | 11 |
| 4 | 7 | 11 | {11, 11} | 11 |

**Complejidad:** O(n log m) en tiempo y O(m) en espacio, con m = min(lathes, n). Recorrer todos los tornos para cada trabajo cuesta O(n·m), hasta 10¹⁰ pasos.
