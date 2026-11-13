Comparar distancias no necesita raíces cuadradas: compara x² + y², con exactitud y en enteros.
---
Ordena cada puesto por la pareja (distancia al cuadrado, índice). La respuesta son las k parejas menores, en orden.
---
O bien ordenas las n parejas (O(n log n)), o bien mantienes un montículo de máximos con los k mejores vistos (O(n log k)) y después ordenas esos k. Elegir el mínimo k veces recorriéndolo todo cada vez es O(n·k), demasiado lento.
