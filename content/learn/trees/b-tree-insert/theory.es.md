## La idea

Un **árbol B** guarda varias claves ordenadas en cada nodo, y un nodo con `k` claves tiene `k + 1` hijos, uno por cada hueco entre sus claves y a sus lados. Este tema permite como mucho **2 claves por nodo**, lo que lo convierte en un **árbol 2-3**. Los árboles B de las bases de datos y los sistemas de archivos guardan cientos de claves por nodo, del tamaño de una página de disco, así que una búsqueda lee solo unas pocas páginas incluso entre miles de millones de claves.

insert baja desde la raíz, entrando en cada nodo en el hijo del hueco donde cae la clave (las claves iguales van a la derecha), hasta llegar a una hoja, y mete la clave en esa hoja en orden. Si la hoja tiene ahora **3 claves**, se **divide**: la clave central sube al padre, y las claves de cada lado forman dos nodos separados. El padre puede tener ahora 3 claves y dividirse a su vez, y así hacia arriba por el camino. Cuando se divide la raíz, su clave central pasa a ser una **raíz nueva** con una clave y dos hijos.

Así el árbol solo crece por arriba, y todas las hojas están siempre a la misma profundidad.

### La entrada por defecto

10, 20, 30, 40, 50, 60, 70, 25. 30 deja la primera hoja en 10 20 30, que se divide: 20 pasa a ser una raíz nueva sobre las hojas 10 y 30. 50 forma 30 40 50, que se divide y sube 40, así que la raíz guarda 20 40 sobre las hojas 10, 30 y 50. 70 hace que 50 60 70 se divida y suba 60; la raíz, ahora 20 40 60, también se divide, y 40 es la raíz nueva sobre 20 (hojas 10 y 30) y 60 (hojas 50 y 70). Por último, 25 baja 40 → 20 → la hoja 30 y se une a ella como 25 30. La salida recorre los nodos nivel a nivel, cada uno seguido de un 0: 40 0, luego 20 0 60 0, luego 10 0 25 30 0 50 0 70 0.

## Pasos

1. Empieza con una hoja vacía como raíz.
2. En cada nodo interno, pasa de largo cada clave que no sea mayor que la nueva y entra en ese hijo.
3. En la hoja, mete la clave en orden.
4. Si un nodo tiene 3 claves, divídelo: su primera y su última clave forman dos nodos, y la clave central va al padre entre ellos.
5. Si se divide la raíz, crea una raíz nueva que solo guarde la clave central.
6. Escribe los nodos nivel a nivel, de izquierda a derecha, cada uno seguido de 0.

## Coste

| Operación | Coste |
|---|---|
| insert | `O(log n)`: un camino de bajada y divisiones a la vuelta |
| Una división | `O(1)` aquí; `O(m)` con `m` claves por nodo |
| Altura | entre unas `log₃ n` (nodos llenos) y `log₂ n` |
| Espacio extra | `O(n)` para los nodos |

## Errores comunes

- **Olvidar los hijos al dividir.** Un nodo interno que se divide también reparte sus hijos: la parte izquierda se queda con los que están a la izquierda de la clave central y la derecha con el resto.
- **Meter una clave nueva en un nodo interno.** Las claves nuevas siempre van a una hoja; un nodo interno solo recibe la clave que sube una división.
- **Hacer crecer el árbol por abajo.** Una hoja nueva bajo una llena rompe la igualdad de profundidad de las hojas; divide en su lugar.

## Cuándo usarlo

Usa un árbol B cuando los datos viven en disco o el acceso a memoria es el cuello de botella: los índices de las bases de datos y los sistemas de archivos leen un nodo por página. Los nodos anchos también aprovechan la caché, y por eso `BTreeMap` de Rust es un árbol B. El árbol 2-3 de aquí es el caso más pequeño, el mejor para ver las divisiones.

## Temas relacionados

- [ABB: buscar, insertar, borrar](learn:trees/bst-operations): el árbol de una clave por nodo que un árbol B generaliza.
- [Árbol rojinegro: inserción](learn:trees/red-black-insert): un árbol 2-3 dibujado con nodos binarios y enlaces rojos.
- [Árbol AVL: rotaciones](learn:trees/avl-rotations): equilibrio con rotaciones en vez de divisiones.
- [Recorrido por niveles](learn:trees/level-order): el recorrido fila a fila que escribe la salida.
