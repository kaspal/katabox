Las sumas prefijas responden totales al instante, pero un solo `"set"` te obligaría a reconstruirlas. Necesitas una estructura barata de actualizar **y** de consultar.
---
Un árbol de Fenwick (árbol indexado binario) guarda sumas parciales de modo que tanto "sumar d en la posición i" como "total de las posiciones 0 a i" cuestan O(log n).
---
Guarda las cantidades actuales en un arreglo. Para `"set"`, suma `count − actual[i]` en `i` en el árbol de Fenwick. Para `"total"`, responde `prefix(b + 1) − prefix(a)`.
