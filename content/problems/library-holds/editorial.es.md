## Enfoque: colas con borrado perezoso y tiques

Mantén, para cada libro, una **cola** de entradas en espera, y un mapa hash `esperando[(libro, socio)]` con el **tique** actual del socio, un número que crece con cada reserva.

- `hold(libro, socio)`: saca un tique nuevo `t`; pon `esperando[(libro, socio)] = t`; añade `(socio, t)` al final de la cola del libro.
- `cancel(libro, socio)`: borra `esperando[(libro, socio)]` si existe. La cola no se toca.
- `return(libro)`: saca entradas del principio de la cola del libro hasta encontrar una válida, es decir, con `esperando[(libro, socio)] == t`. Ese socio se lleva el ejemplar: borra su entrada del mapa e informa de él. Si la cola se acaba, informa de `-1`.

Las entradas de los socios que cancelaron se tiran la primera vez que una devolución llega a ellas (**borrado perezoso**). Los tiques importan cuando un socio cancela y vuelve a reservar: la entrada antigua `(socio, t_viejo)` sigue en la cola, delante de la nueva, pero el mapa guarda ahora `t_nuevo`, así que la antigua se salta y el socio conserva su nuevo sitio al final.

### Paso a paso

Ejemplo 3: las reservas dan al 7 el tique 1 y al 8 el tique 2. El socio 7 cancela y vuelve a reservar con el tique 3, así que la cola es (7, 1), (8, 2), (7, 3). La primera devolución salta (7, 1) (caducada) y da el ejemplar al 8; la segunda se lo da al 7 por (7, 3). Informes: 8, 7.

**Complejidad:** cada entrada entra una vez en la cola y sale como mucho una vez, así que O(ops) esperado en total, y O(ops) de memoria. Quitar enseguida de una lista a quien cancela cuesta O(longitud de la lista) por cancelación, O(ops²) en el peor caso.
