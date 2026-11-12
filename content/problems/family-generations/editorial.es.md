## Enfoque: listas de hijos y un recorrido por niveles

El arreglo de padres le dice a cada persona quién está por encima, pero para medir el árbol desde arriba hace falta la dirección contraria. Construye `hijos[p]`, la lista de todos los que tienen a `p` como padre, en una pasada, y apunta quién es la fundadora (la persona con `-1`).

Después recorre el árbol **generación a generación** (búsqueda en anchura): la generación actual empieza siendo `[fundadora]`; la siguiente generación son todos los hijos de la actual. Cada ronda suma una generación, y el recorrido termina cuando una generación queda vacía.

```
hijos[p] = [i con parent[i] == p]
nivel = [fundadora]; cuenta = 0
mientras nivel:
    cuenta += 1
    nivel = [h para p en nivel para h en hijos[p]]
devolver cuenta
```

### Paso a paso

`parent = [2, -1, 1]`: hijos de 1 = [2], hijos de 2 = [0]. El recorrido visita [1], luego [2], luego [0] y después nada: 3 generaciones.

**Complejidad:** O(n) de tiempo y espacio: cada persona aparece una vez como hija y se visita una vez. Subir desde cada persona por separado cuesta O(n · profundidad), hasta unos n²/2 pasos en una sola línea de descendientes.
