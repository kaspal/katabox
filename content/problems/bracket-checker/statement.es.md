Dmitri añadió una comprobación de corchetes a su pequeño editor de código para que avise de los fragmentos desequilibrados. Detecta los que se cierran en el orden equivocado, pero algunos fragmentos rotos siguen pasando la revisión sin problemas.

Dada una cadena `code` formada solo por los caracteres `()[]{}`, devuelve `true` si sus corchetes están equilibrados y `false` en caso contrario. Una cadena está equilibrada cuando cada corchete de cierre empareja con el último corchete de apertura sin emparejar, que debe ser del mismo tipo, y al final no queda ningún corchete de apertura sin cerrar.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
code = "{[()]}"
Salida: true
```

### Ejemplo 2

```
code = "{()}[]"
Salida: true
```

### Ejemplo 3

```
code = "()({[)"
Salida: false
```

El `)` del final intenta cerrar el `[`, que es el último corchete de apertura sin emparejar.

### Restricciones

- 1 ≤ code.length ≤ 10⁵
- code solo contiene los caracteres `()[]{}`.
