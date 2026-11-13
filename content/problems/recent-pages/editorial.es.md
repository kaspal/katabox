## Enfoque: un mapa hash más un orden de uso

Dos operaciones tienen que ser O(1): encontrar una página por su número, y mantener las páginas en orden de último uso para poder quitar la menos reciente.

- Un **mapa hash** de página a su entrada encuentra las páginas.
- Una **lista doblemente enlazada** (o un mapa con orden de inserción) mantiene el orden: una página usada se desengancha y se pone en el extremo "reciente"; desalojar quita del extremo "antiguo".

Para `"get"`: si la página está, muévela al extremo reciente e informa su valor; si no, informa −1. Para `"put"`: si está, actualiza y mueve; si no, añádela en el extremo reciente y, si la caché tiene ahora más de `capacity` páginas, quita la más antigua.

Las referencias usan lo que ofrece cada lenguaje: el `OrderedDict` de Python (`move_to_end`, `popitem(last=False)`), el `Map` de JavaScript y los arrays de PHP (los dos conservan el orden de inserción: borra y vuelve a insertar para refrescar; la primera clave es la más antigua), y en Rust un mapa más un `BTreeMap` ordenado por un contador de usos.

**Complejidad:** O(1) por operación (O(log n) con el mapa ordenado de Rust). Buscar en una lista simple es O(capacity) por operación.
