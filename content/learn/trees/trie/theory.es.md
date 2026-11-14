## La idea

Un **trie** (o árbol de prefijos) guarda palabras letra a letra. Cada nodo representa un prefijo y cada arista que sale de él es una letra: desde la raíz, sigue `c`, luego `a`, luego `r`, y llegas al nodo de "car". Las palabras que empiezan igual comparten el mismo camino, así que "car", "cart", "care" y "cat" pasan todas por los nodos de "c" y "ca".

Aquí cada nodo guarda dos cosas:

- un **contador**: cuántas palabras insertadas pasan por él, es decir, cuántas empiezan por su prefijo;
- una **marca de fin**: si alguna palabra termina justo aquí.

### Insertar y contar

Para **insertar** una palabra, empieza en la raíz y sigue una letra cada vez. Si falta una arista, crea un nodo nuevo para ella. Suma 1 al contador de cada nodo en el que entras y marca el último como final.

Para **contar las palabras con un prefijo**, recorre el prefijo igual. Si falta una arista, ninguna palabra empieza así: la respuesta es 0. Si no, el nodo al que llegas ya tiene la respuesta en su contador, y su marca de fin dice si el prefijo es una palabra.

### La entrada por defecto

Se insertan las palabras car, cart, care, cat y dog, y después se cuenta el prefijo "ca". Insertar cart después de car sigue tres aristas que ya existen y crea un solo nodo, el de `t`. Con las cinco dentro, el trie tiene 9 nodos bajo la raíz, el nodo de "car" tiene contador 3 (car, cart, care) y el nodo de "ca" tiene contador 4. "ca" no es una palabra, así que la salida es `[4, 0]`.

## Pasos

1. Empieza con un nodo raíz vacío.
2. Inserta cada palabra: para cada letra, crea el hijo si falta, pasa a él y suma 1 a su contador.
3. Tras la última letra, marca el nodo como final de palabra.
4. Contar un prefijo: sigue sus letras desde la raíz. Si falta una arista, devuelve `[0, 0]`.
5. Si no, devuelve el contador del nodo, y 1 si está marcado como final, si no 0.

## Coste

| Operación | Coste |
|---|---|
| Insertar una palabra de longitud `L` | `O(L)` |
| Contar un prefijo de longitud `L` | `O(L)`, haya las palabras que haya |
| Espacio extra | `O(total de letras)` en el peor caso; los prefijos compartidos se guardan una vez |

## Errores comunes

- **Contar en vez de marcar.** El contador dice cuántas palabras *pasan* por un nodo; no dice si alguna termina allí. Guarda las dos cosas.
- **Insertar una palabra dos veces.** Se cuenta dos veces. Mira antes la marca de fin si las palabras tienen que ser únicas.
- **La memoria.** Cada nodo guarda un mapa de hijos, mucho más pesado que un carácter.

## Cuándo usarlo

Usa un trie cuando las preguntas sean sobre prefijos: autocompletar, contar o listar las palabras que empiezan por algo, corregir la ortografía o encontrar el prefijo conocido más largo (los routers emparejan así prefijos de direcciones IP).

## Temas relacionados

- [Tabla hash: encadenamiento](learn:hashing/separate-chaining): más rápida para buscar palabras enteras, inútil para prefijos.
- [Preorden, inorden y postorden](learn:trees/tree-traversals): recorrer un trie en preorden lista sus palabras en orden si visitas los hijos por orden de letra.
