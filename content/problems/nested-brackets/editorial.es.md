## Enfoque: una pila de corchetes abiertos

Recorre `text` una vez saltando las letras. Apila cada corchete de apertura. Ante uno de cierre, el corchete abierto más interno es el tope de la pila: si la pila está vacía o el tope es de otro tipo, el texto no está bien anidado; si no, desapílalo. Al terminar, cualquier corchete que quede en la pila nunca se cerró.

### Paso a paso

`text = "({)}"`:

| carácter | pila antes | acción |
| --- | --- | --- |
| `(` | [] | apilar |
| `{` | [`(`] | apilar |
| `)` | [`(`, `{`] | el tope es `{`, no `(`: devolver false |

**Complejidad:** O(n) en tiempo y O(n) de espacio extra para la pila. Borrar una y otra vez los pares contiguos como `()` hasta que nada cambie también es correcto, pero con un anidamiento profundo cuesta O(n²).
