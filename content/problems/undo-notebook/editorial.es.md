## Enfoque: un mapa y dos pilas de registros de cambio

Guarda `valores`, un mapa hash de clave a valor actual, y dos pilas de **registros de cambio** `(clave, antiguo, nuevo)`, donde `antiguo` es el valor antes de la escritura (o "ninguno").

- `write(k, v)`: apila `(k, valores.get(k), v)` en `deshacer`, pon `valores[k] = v` y vacía `rehacer`.
- `read(k)`: informa de `valores.get(k, -1)`.
- `undo`: desapila `(k, antiguo, nuevo)` de `deshacer`; restaura `antiguo` (borra la clave si es "ninguno"); apila el registro en `rehacer`.
- `redo`: desapila `(k, antiguo, nuevo)` de `rehacer`; pon `valores[k] = nuevo`; apila el registro en `deshacer`.

Deshacer restaura exactamente el estado anterior a la escritura que quita, porque las escrituras posteriores ya se han deshecho (orden de pila). Así que el `antiguo` de cada registro es correcto cuando se usa. Vaciar `rehacer` en una escritura nueva es lo que pide el enunciado: el futuro deshecho ya no vale.

### Paso a paso

Ejemplo 3: write (1, 5) apila (1, ninguno, 5); undo quita la clave 1 y pasa el registro a rehacer; write (2, 6) vacía rehacer; redo no hace nada. Lecturas: clave 1 → -1, clave 2 → 6.

**Complejidad:** O(1) esperado por operación y O(ops) de memoria. Repetir todo el historial de escrituras en cada lectura cuesta O(ops) por lectura, O(ops²) en total.
