## Enfoque: una pila de nombres de carpeta

Divide la ruta por cada `/`. La pila guarda las carpetas desde la superior hasta donde estás:

- parte vacía o `.`: no cambia nada;
- `..`: saca la última carpeta, salvo que la pila esté vacía (estás arriba del todo);
- cualquier otra cosa: métela, aunque esté hecha de puntos (`...` es un nombre).

Al final, la ruta limpia es `/` + la pila unida con `/` (solo `/` si la pila está vacía).

### Paso a paso

`/photos/./2019//summer/../winter/` se divide en `"", photos, ., 2019, "", summer, .., winter, ""`. La pila pasa por `[photos]`, `[photos, 2019]`, `[photos, 2019, summer]`, vuelve a `[photos, 2019]` y termina en `[photos, 2019, winter]`.

**Complejidad:** O(longitud) en tiempo y espacio.
