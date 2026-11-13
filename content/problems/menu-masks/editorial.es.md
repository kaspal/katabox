## Enfoque: cada conjunto como una máscara de bits

Un número `mask` entre 1 y 2ⁿ − 1 describe un conjunto de platos: el bit `i` a 1 significa que el plato `i` está dentro. Recorrer todas las máscaras recorre todos los conjuntos no vacíos.

Para cada máscara, suma los precios de los platos cuyo bit está a 1 (comprobando `mask >> i & 1` para cada `i`). Si el total es igual al presupuesto, anota la lista de esos `i` en orden creciente.

El orden de las máscaras no es el pedido: la máscara 0b1001 (platos 0 y 3) va después de 0b0110 (platos 1 y 2), pero `[0, 3]` va antes que `[1, 2]`. Así que ordena las listas anotadas lexicográficamente al final. (En PHP, `sort` compara los arrays primero por longitud, así que hace falta una comparación propia.)

**Complejidad:** O(n · 2ⁿ) en tiempo, unos 5·10⁵ pasos para n = 15, más la ordenación de las respuestas.
