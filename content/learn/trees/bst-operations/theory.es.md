## La idea

Un **árbol binario de búsqueda (ABB)** guarda claves en nodos de forma que, en cada nodo, las claves del subárbol izquierdo son menores y las del subárbol derecho no lo son. Esa única regla permite que cada operación recorra un solo camino desde la raíz en vez de mirar todos los nodos. insert baja (a la izquierda si la clave es menor, si no a la derecha) y engancha una hoja nueva donde se acaba el camino; find baja igual y responde 1 o 0.

### Borrar

Un nodo **sin hijos o con un hijo** es fácil: su hijo (o nada) ocupa su lugar, enganchado a su padre.

Un nodo con **dos hijos** no se puede quitar sin más. En su lugar, busca su **sucesor**, la clave más pequeña de su subárbol derecho (una vez a la derecha y luego a la izquierda todo lo posible). Encaja entre los dos lados. Copia la clave del sucesor en el nodo y después borra el sucesor del subárbol derecho; no tiene hijo izquierdo, así que ese borrado es el caso fácil.

En la entrada por defecto se hace insert 50, 30, 70, 60, 80, 65 y luego delete 50, que tiene dos hijos. Su sucesor es 60 (a la derecha hasta 70, a la izquierda hasta 60). 60 se copia en la raíz, y el antiguo nodo 60, cuyo único hijo es 65, se quita, así que 65 pasa a ser el hijo izquierdo de 70.

### Altura

Cada operación cuesta un recorrido hacia abajo, así que su coste es la **altura** `h` del árbol. Claves insertadas en orden aleatorio dan una altura cercana a `log n`. Claves ordenadas forman un árbol **degenerado**, en realidad una lista enlazada, de altura `n`. Los árboles equilibrados (próximamente) se recolocan para evitarlo.

## Pasos

1. insert `k`: baja desde la raíz (a la izquierda si `k` es menor, si no a la derecha) y engancha `k` como hoja nueva.
2. find `k`: baja igual hasta encontrar `k` (devuelve 1) o quedarte sin nodos (devuelve 0).
3. delete `k`: baja hasta `k`. Si tiene un hijo como mucho, cámbialo por ese hijo.
4. Si tiene dos hijos, copia en él la clave de su sucesor y borra el sucesor del subárbol derecho.
5. Al final, la salida tiene las respuestas de find, un 0 y luego las claves en orden.

## Coste

| Operación | Coste |
|---|---|
| insert, find, delete | `O(h)` para una altura `h` |
| Árbol equilibrado | `O(log n)` |
| Árbol degenerado | `O(n)` |
| Espacio extra | `O(n)` para los nodos |

## Errores comunes

- **Olvidar volver a enlazar.** Un borrado tiene que actualizar el enlace del padre (o la raíz) hacia el sustituto; el ejemplo lo hace devolviendo la nueva raíz del subárbol.
- **Claves iguales sin criterio.** Decide adónde van las claves iguales (aquí, a la derecha) y usa la misma regla en insert, find y delete.

## Cuándo usarlo

Usa un ABB cuando necesites un conjunto o un mapa que además se mantenga ordenado: la clave menor o mayor, la siguiente clave tras un valor o todas las claves de un rango.

## Temas relacionados

- [Búsqueda binaria](learn:searching/binary-search): la misma división por la mitad, en un arreglo ordenado.
- [Preorden, inorden y postorden](learn:trees/tree-traversals): el inorden lee un ABB ordenado.
- [Montículo binario y cola de prioridad](learn:trees/binary-heap): otro orden de árbol, solo para la clave menor.
- Rotaciones AVL (próximamente): mantener la altura cerca de `log n`.
