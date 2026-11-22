## Idea

**Lo voraz es óptimo.** Corta cada bloque tan largo como se pueda. El primer bloque de
cualquier división válida acaba como muy tarde donde acaba el voraz, y un tramo sin
repeticiones sigue sin repeticiones si se acorta, así que por inducción los cortes voraces
van siempre al menos igual de avanzados que los de cualquier otra división: nunca usan más
bloques.

**El siguiente corte.** Un bloque que empieza en `i` puede llegar hasta la emisión `r` justo
cuando el tramo sin repeticiones más largo que termina en `r` empieza en `i` o antes, es
decir `start[r] ≤ i` (con `start` de la ventana del problema base, que nunca baja). Así
`next[i]`, la primera emisión después del bloque más largo desde `i`, es la primera `r` con
`start[r] > i` (o `n`); un solo puntero las encuentra todas.

**Muchas peticiones.** Construye `jump[0] = next` y `jump[k][i] = jump[k−1][jump[k−1][i]]`,
con `n` saltando a sí mismo. Para una petición `[a, b]`, empieza en `a` con un bloque; para
`k` de mayor a menor, si `jump[k][pos] ≤ b`, ve allí y suma `2^k` bloques. La cuenta final
es la respuesta.

## Complejidad

O(n log n) de tiempo y memoria para construir los saltos, O(log n) por petición. Recorrer
los bloques voraces en cada petición cuesta hasta O(n) cada una, 10¹⁰ pasos en total.
