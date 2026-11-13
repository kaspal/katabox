## Enfoque: BFS sobre máscaras de habilidades

Solo importa el conjunto de habilidades cubiertas, así que los estados son las máscaras `0 .. 2^skills − 1`. Desde el estado `m`, añadir al voluntario `p` lleva a `m | people[p]` y cuesta un miembro del equipo.

Haz un BFS desde la máscara 0 (nadie todavía, nada cubierto). La primera vez que se alcanza la máscara completa `2^skills − 1`, su distancia es el tamaño del equipo más pequeño. Si el BFS termina sin alcanzarla, devuelve −1.

Los voluntarios repetidos llevan a los mismos movimientos, así que quitarlos primero ahorra tiempo.

### Paso a paso

Ejemplo 1, máscara completa 1111: desde 0, un voluntario llega a 0011, 0100, 1100 o 1001. Desde 0011, añadir 1100 llega a 1111: distancia 2.

**Complejidad:** O(2^skills · n), unos 10⁶ pasos. Probar equipos por tamaño es O(C(n, k)) para cada tamaño k, lo que explota cuando la respuesta es grande (un equipo de 14 entre 60 voluntarios).
