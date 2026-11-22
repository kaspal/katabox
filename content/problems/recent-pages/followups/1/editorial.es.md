## Idea

El problema base necesitaba un solo orden por recencia; aquí la víctima es la página menos
usada, y la recencia solo deshace empates. Guarda tres cosas: un mapa de página a valor y
número de usos, una lista ordenada por recencia para cada número de usos (una lista
enlazada con un mapa a sus nodos, o un mapa hash que conserve el orden de inserción) y
`least`, el menor número de usos que tiene alguna página.

Un uso mueve la página de la lista `f` al final de la lista `f + 1`; si la lista `f` se
queda vacía y `f` era `least`, entonces `least = f + 1` (la página acaba de llegar ahí).
Una página nueva va al final de la lista 1 y fija `least = 1`, después de expulsar, si la
caché está llena, la primera página (la más antigua) de la lista `least`. Solo una página
nueva puede bajar `least`, y lo baja a 1, así que `least` nunca hay que buscarlo.

## Complejidad

O(1) por operación (esperado, con mapas hash), O(ops.length) en total, y O(capacity) de
memoria extra.
