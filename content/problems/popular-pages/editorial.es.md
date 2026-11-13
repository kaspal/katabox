## Enfoque: cubos por recuento de usos, cada uno en orden de uso

La página que hay que quitar es el mínimo de `(recuento de usos, último uso)`. Guarda:

- `value[page]` y `count[page]`;
- `bucket[c]`: las páginas con recuento `c`, en orden de último uso, la más antigua primero (un conjunto con orden de inserción, o una lista enlazada);
- `minCount`: el menor `c` con un cubo no vacío.

**Usar una página** (un `"get"` que la encuentra, o un `"put"` sobre una página guardada): quítala de `bucket[c]`, añádela al final de `bucket[c + 1]`, y si `bucket[c]` quedó vacío y `c = minCount`, pon `minCount = c + 1`. Añadir al final mantiene cada cubo en orden de uso.

**Añadir una página** a una caché llena: quita la primera página (la más antigua) de `bucket[minCount]`. Después añade la página nueva a `bucket[1]` y pon `minCount = 1`.

Cada paso es O(1) con mapas hash y conjuntos ordenados. (La referencia en Rust guarda en su lugar un conjunto ordenado de `(recuento, último uso, página)`, O(log n) por paso, que es más sencillo de escribir en Rust.)

### Paso a paso

Ejemplo 1: tras `put 3`, `put 4`, `get 3`, los cubos son {1: [4], 2: [3]}. `put 5` quita el 4 (cubo 1). `get 5` pasa el 5 al cubo 2: {2: [3, 5]}, minCount 2. `put 6` quita el 3, el más antiguo del cubo 2.

**Complejidad:** O(1) por operación. Recorrer todas las páginas guardadas en cada desalojo es O(capacity) por operación.
