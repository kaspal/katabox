## Enfoque: dos punteros que saltan los símbolos

Usa un índice `i` desde la izquierda y otro `j` desde la derecha.

1. Mueve `i` a la derecha mientras `text[i]` no sea letra ni dígito, y `j` a la izquierda igual (sin que se crucen).
2. Si `i < j`, compara `text[i]` y `text[j]` en minúscula. Si son distintos, la cinta no es espejo.
3. Si no, acerca ambos índices y repite.

Cuando los índices se encuentran, todos los pares coincidieron.

### Paso a paso

`text = "Top spot: 7 -- 7 tops pot"` compara `T`/`t`, `o`/`o`, `p`/`p`, `s`/`s`, … y por último `7`/`7`; los espacios, los dos puntos y los guiones se saltan.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Construir una copia limpia y su reverso también es O(n), pero necesita O(n) de memoria extra.
