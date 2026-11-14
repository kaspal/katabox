## La idea

El **direccionamiento abierto** guarda una tabla hash en un simple arreglo de huecos, una clave por hueco, sin listas. Cada clave tiene un **hueco de origen**, aquí `key mod 8`. Si otra clave ocupa ese hueco, la tabla **sondea**: con el **sondeo lineal** prueba el hueco siguiente, y el siguiente, dando la vuelta al llegar al final, hasta encontrar la clave o un hueco vacío.

insert recorre el camino de sondeo y pone la clave en el primer hueco vacío. find recorre el mismo camino: si encuentra la clave, la ha encontrado (1); si llega a un hueco vacío, la clave no puede estar más adelante, así que no está (0).

### Lápidas

Borrar es lo delicado. Si delete simplemente vaciara el hueco, una búsqueda posterior de una clave guardada *más allá* se pararía en el hueco y respondería 0 por error. Por eso delete deja una **lápida** (✕): una marca que dice "aquí hubo algo, sigue sondeando".

El ejemplo lo mantiene mínimo: insert nunca reutiliza lápidas y admite como mucho 7 inserciones, así que siempre queda al menos un hueco vacío y todo sondeo se detiene. Las tablas reales sí reutilizan las lápidas para claves nuevas, y **redimensionan** (reservan un arreglo mayor y recalculan el hash de cada clave viva) cuando la tabla se llena demasiado.

### Agrupamiento

La entrada por defecto inserta 3, 11, 19 y 4. Las tres primeras tienen el hueco de origen 3, así que llenan los huecos 3, 4 y 5. La clave 4 tiene el hueco de origen 4, que está ocupado, y acaba en el hueco 6. Las rachas de huecos ocupados como esta son **grupos**: cualquier clave cuyo hash caiga dentro se une al grupo y lo alarga. find 27 (origen 3) tiene que pasar por las cuatro claves antes de llegar al hueco vacío 7.

## Pasos

1. Empieza con 8 huecos vacíos.
2. Calcula el hueco de origen, `i = key mod 8`.
3. Mientras el hueco `i` no esté vacío ni tenga la clave, avanza: `i = (i + 1) mod 8`.
4. insert: pon la clave en el hueco `i`. find: devuelve 1 si el hueco `i` tiene la clave, si no 0.
5. delete: si el hueco `i` tiene la clave, cámbiala por una lápida.

## Coste

| Operación | Coste |
|---|---|
| insert, find, delete, en promedio | `O(1)` mientras la tabla no esté demasiado llena |
| insert, find, delete, peor caso | `O(m)`: un único grupo largo |
| Espacio extra | `O(m)` para `m` huecos |

## Errores comunes

- **Vaciar un hueco al borrar.** Rompe todos los caminos de sondeo que pasaban por él. Deja una lápida.
- **Dejar que la tabla se llene.** Sin ningún hueco vacío, buscar una clave que no está da vueltas para siempre. Redimensiona mucho antes.
- **Ignorar los grupos.** El sondeo lineal se frena mucho al llenarse la tabla; los saltos cuadráticos o un segundo hash separan más las claves.

## Cuándo usarlo

El direccionamiento abierto guarda las claves directamente en el arreglo, así que los sondeos leen memoria contigua y aprovechan la caché. El `dict` de Python lo usa. Prefiere el encadenamiento cuando haya muchos borrados o la tabla tenga que ir casi llena.

## Temas relacionados

- [Tabla hash: encadenamiento](learn:hashing/separate-chaining): las claves que colisionan comparten una lista.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): la misma vuelta al principio con `mod`.
- [Filtro de Bloom](learn:hashing/bloom-filter): hashing que solo responde "quizá" o "no".
