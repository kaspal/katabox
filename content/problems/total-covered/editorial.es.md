## El error

El recorrido mantiene un bloque actual `[cur_s, cur_e]` y, en cada turno, o alarga su fin o lo cierra y abre otro. Eso solo funciona si los turnos llegan ordenados por su inicio, porque el inicio del bloque nunca retrocede. El código inicial ordena por la clave equivocada:

```python
    for s, e in sorted(shifts, key=lambda x: x[1]):
```

Ordenado por el fin, un turno que empieza antes que el bloque actual puede llegar después. Como se solapa, solo se actualiza `cur_e`, y el tiempo entre su inicio y `cur_s` nunca se cuenta.

Con `shifts = [[1, 10], [2, 3]]`, el orden queda `[2, 3]`, `[1, 10]`. El bloque empieza como 2 a 3, luego `[1, 10]` se solapa y lo estira a 2 a 10, así que el código devuelve 8. La respuesta correcta es 9 (de 1 a 10).

## La corrección

Ordena por el inicio (el orden por defecto de las listas en Python compara primero el inicio):

```python
    for s, e in sorted(shifts):
```

En TypeScript, ordena con `(x, y) => x[0] - y[0]` en lugar de comparar los finales.

**Complejidad:** O(n log n) en tiempo por la ordenación y O(n) de espacio extra para la copia ordenada.
