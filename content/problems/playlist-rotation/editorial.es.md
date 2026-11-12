## Enfoque: contar y cortar una sola vez

Pulsar el botón `n` veces (n canciones) deja cada canción en su sitio, así que solo importan `r = k mod n` pulsaciones. Después de ellas, las últimas `r` canciones van delante y las demás detrás, en su orden original. Eso es un solo corte en la lista:

1. Recorre hasta la cola contando `n`. Una lista vacía se devuelve tal cual.
2. Calcula `r = k mod n`. Si `r = 0`, no cambia nada.
3. Avanza `n − r − 1` pasos desde la cabeza hasta la **nueva cola**. El nodo siguiente es la **nueva cabeza**.
4. Haz `nuevaCola.next = null` y `colaAntigua.next = cabezaAntigua`.

No se crea ni se copia ningún nodo: solo cambian dos enlaces.

### Paso a paso

`playlist = [4, 8, 15, 16, 23]`, `k = 2`: n = 5, r = 2. La nueva cola está 3 − 1 = 2 pasos después de la cabeza, el nodo 15; la nueva cabeza es el 16. Cortando tras el 15 y enlazando 23 → 4 queda `[16, 23, 4, 8, 15]`.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Pulsar el botón `r` veces una a una cuesta un recorrido hasta el final cada vez, O(n·r), que es O(n²) cuando r ronda n/2: demasiado lento para 10⁵ canciones.
