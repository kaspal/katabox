Una app de cuaderno de laboratorio guarda un número por clave de experimento y permite deshacer y rehacer. Procesa una lista de operaciones, donde `ops[i]` es el nombre de la operación y `args[i]` sus números:

- `"write"`, args `[key, value]`: fija el valor de la clave. Esto también **borra** todo lo que se podía rehacer.
- `"read"`, args `[key]`: **informa** del valor actual de la clave, o de `-1` si no tiene.
- `"undo"`, args `[]`: deshace la escritura más reciente que sigue en vigor y devuelve a la clave su valor anterior (o ningún valor). Si no hay nada que deshacer, no pasa nada.
- `"redo"`, args `[]`: vuelve a aplicar la última escritura deshecha. Si no hay nada que rehacer, no pasa nada.

Devuelve los informes de las operaciones `"read"`, en orden.

### Ejemplo 1

```
ops = ["write", "write", "read", "undo", "read", "redo", "read", "undo", "undo", "read"], args = [[1, 10], [1, 20], [1], [], [1], [], [1], [], [], [1]]
Salida: [20, 10, 20, -1]
```

Tras deshacer las dos escrituras, la clave 1 vuelve a no tener valor.

### Ejemplo 2

```
ops = ["read", "undo", "redo"], args = [[7], [], []]
Salida: [-1]
```

### Ejemplo 3

```
ops = ["write", "undo", "write", "redo", "read", "read"], args = [[1, 5], [], [2, 6], [], [1], [2]]
Salida: [-1, 6]
```

La segunda escritura borra la escritura deshecha de la clave 1, así que `redo` no tiene nada que hacer.

### Restricciones

- 1 ≤ ops.length ≤ 1.5·10⁵, args.length = ops.length
- Cada `ops[i]` es `"write"`, `"read"`, `"undo"` o `"redo"`, con los números descritos arriba
- 1 ≤ key ≤ 10⁹, 0 ≤ value ≤ 10⁹
