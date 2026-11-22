Comprobar cada parte con una pila supone O(n²) partes, cada una revisada en hasta O(n) pasos. Una parte equilibrada nunca puede contener un corchete de cierre que no encuentre pareja dentro de ella. ¿Qué hacen esos corchetes con el texto?
---
Usa la pila de siempre, pero guarda **posiciones** en lugar de corchetes. Cuando un corchete de cierre empareja con el de arriba, la parte que acaba de cerrarse empieza justo después de la posición que queda ahora en la cima.
---
Empieza la pila con `-1`, un muro antes del texto. Apila la posición de cada corchete que abre. Ante un corchete de cierre que empareja con el de la posición de la cima, desapila y mide `i − cima`. Ante cualquier otra cosa (nada que emparejar, o un tipo distinto), la posición `i` pasa a ser el nuevo muro: deja en la pila solo `i`.
