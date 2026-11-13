## Enfoque: expandir alrededor de cada centro

Un trozo que se lee igual al revés es simétrico respecto a su centro, que es una letra (longitud impar) o un hueco entre dos vecinas (longitud par). Para cada uno de los 2n − 1 centros, crece hacia fuera mientras las letras de los dos lados sean iguales; el mayor crecimiento es el trozo espejo más largo con ese centro.

Procesa los centros de izquierda a derecha (letra 0, hueco 0-1, letra 1, hueco 1-2, …) y solo sustituye el mejor trozo por uno estrictamente más largo. Entre trozos de igual longitud, un centro posterior significa un inicio posterior, así que gana el que empieza antes.

(El algoritmo de Manacher encuentra la misma respuesta en O(n); la referencia en Python lo usa, porque expandir en Python puro es lento con largas series de una misma letra.)

### Paso a paso

"noonmadam": el hueco entre las dos `o` crece hasta "noon" (4); la `d` crece hasta "madam" (5). No hay nada más largo, así que "madam".

**Complejidad:** O(n²) en tiempo en el peor caso (un texto de una sola letra repetida), con espacio extra O(1). Comprobar cada trozo por separado es O(n³).
