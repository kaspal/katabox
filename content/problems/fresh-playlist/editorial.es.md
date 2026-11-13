## Enfoque: una ventana deslizante con las últimas posiciones

Recorre `end` de izquierda a derecha, manteniendo `start` de forma que `plays[start..end]` no tenga repeticiones, y `last[x]`, la última posición en la que sonó la canción `x`.

Para cada nueva emisión `x = plays[end]`:

- si `x` sonó en `last[x] ≥ start`, esa emisión anterior está dentro de la ventana, así que la ventana solo puede seguir fresca si empieza después de ella: `start = last[x] + 1`;
- anota `last[x] = end` y actualiza la mejor longitud `end − start + 1`.

`start` solo avanza, así que cada emisión entra y sale de la ventana una vez.

### Paso a paso

`[4, 8, 4, 2, 9, 8, 1]`: ventanas `[4]`, `[4, 8]`; el segundo 4 lleva start a 1: `[8, 4]`; después `[8, 4, 2]`, `[8, 4, 2, 9]`; el segundo 8 (visto por última vez en 1) lleva start a 2: `[4, 2, 9, 8]`; después `[4, 2, 9, 8, 1]`, longitud 5.

**Complejidad:** O(n) en tiempo y O(n) de espacio. Hacer crecer un tramo desde cada inicio hasta una repetición es O(n · respuesta), O(n²) cuando los tramos son largos.
