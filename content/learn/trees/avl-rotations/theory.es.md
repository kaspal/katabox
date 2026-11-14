## La idea

Un **árbol AVL** es un árbol binario de búsqueda que se repara tras cada inserción, así que su altura se mantiene cerca de `log n` sea cual sea el orden de las claves. Cada nodo guarda su **altura** (una hoja tiene 1 y un subárbol vacío 0), y en cada nodo el **equilibrio**, altura izquierda menos altura derecha, debe quedarse en −1, 0 o 1.

insert baja como en un ABB normal (las claves iguales van a la derecha) y engancha una hoja nueva. A la vuelta, cada nodo del camino recalcula su altura y su equilibrio. Solo esos nodos pueden romper la regla, y solo por uno, así que un equilibrio malo siempre es +2 o −2. La corrección depende de adónde fue la clave nueva:

- **Izquierda-izquierda** (+2, la clave es menor que la del hijo izquierdo): una **rotación a la derecha**. El hijo izquierdo sube, el nodo pasa a ser su hijo derecho, y el antiguo subárbol derecho del hijo pasa a ser el subárbol izquierdo del nodo.
- **Derecha-derecha** (−2, la clave no es menor que la del hijo derecho): la simétrica, una **rotación a la izquierda**.
- **Izquierda-derecha** y **derecha-izquierda** (la clave fue al lado interior): primero rota el hijo en sentido contrario, lo que endereza el zigzag, y luego rota el nodo.

Una rotación mantiene las claves en el mismo inorden, así que el árbol sigue siendo un ABB válido.

### La entrada por defecto

10, 20, 30, 40, 50, 25. 30 deja 10 en −2, derecha-derecha: una rotación a la izquierda pone arriba 20 con hijos 10 y 30. Después 50 deja 30 en derecha-derecha, y una rotación a la izquierda pone 40 en su lugar con hijos 30 y 50. 25 va a la derecha en 20, a la izquierda en 40 y a la izquierda en 30. La raíz 20 acaba en −2 con la clave en el lado interior: derecha-izquierda. 40 rota a la derecha, lo que sube 30, y luego 20 rota a la izquierda, así que 30 pasa a ser la raíz con 20 (hijos 10 y 25) a su izquierda y 40 (hijo derecho 50) a su derecha. La salida en preorden es 30, 20, 10, 25, 40, 50.

## Pasos

1. Baja como en un ABB y engancha la clave como hoja nueva de altura 1.
2. A la vuelta, pon como altura de cada nodo 1 más la de su hijo más alto, y calcula su equilibrio.
3. Equilibrio +2: si la clave no es menor que la del hijo izquierdo, rota primero el hijo izquierdo a la izquierda; después rota el nodo a la derecha.
4. Equilibrio −2: si la clave es menor que la del hijo derecho, rota primero el hijo derecho a la derecha; después rota el nodo a la izquierda.
5. Al final, escribe las claves en preorden.

## Coste

| Operación | Coste |
|---|---|
| insert | `O(log n)`: un camino de bajada y de vuelta |
| Una rotación | `O(1)`: tres enlaces y dos alturas |
| Altura | como mucho unas `1.44 log n` |
| Espacio extra | `O(n)` para los nodos |

## Errores comunes

- **Actualizar las alturas en mal orden.** Tras una rotación, actualiza primero el nodo que baja: el que sube queda encima de él.
- **Una sola rotación en un zigzag.** En el lado interior, una rotación solo pasa el desequilibrio al otro lado. Endereza primero.
- **Perder la nueva cima.** Una rotación devuelve la nueva cima del subárbol; el enlace del padre (o la raíz) tiene que recogerla.

## Cuándo usarlo

Usa un árbol AVL cuando necesites búsqueda e inserción garantizadas en `O(log n)` sobre un conjunto o mapa ordenado, incluso si las claves llegan ordenadas. Queda algo más plano que un árbol rojinegro, así que las búsquedas son un poco más rápidas, a cambio de más rotaciones.

## Temas relacionados

- [ABB: buscar, insertar, borrar](learn:trees/bst-operations): el árbol simple que un AVL mantiene equilibrado.
- [Árbol rojinegro: inserción](learn:trees/red-black-insert): un equilibrio más flojo mantenido con colores en vez de alturas.
- [Árbol B: inserción y división](learn:trees/b-tree-insert): equilibrio dividiendo nodos que guardan varias claves.
- [Preorden, inorden y postorden](learn:trees/tree-traversals): el recorrido en preorden que escribe la salida.
