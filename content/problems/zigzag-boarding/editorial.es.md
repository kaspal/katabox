## Enfoque: partir, invertir la mitad trasera e intercalar

1. **Centro:** avanza `slow` un paso y `fast` dos pasos hasta que `fast` llegue al final. Con n nodos, la mitad delantera son los primeros ⌈n/2⌉ nodos.
2. **Cortar e invertir:** corta tras la mitad delantera e invierte la trasera sin memoria extra (tres punteros).
3. **Intercalar:** alterna un nodo de la mitad delantera y otro de la trasera invertida, reenlazándolos sobre la marcha.

### Paso a paso

`[1, 2, 3, 4, 5]`: delantera `[1, 2, 3]`, trasera `[4, 5]`, invertida `[5, 4]`. Al intercalar queda `1, 5, 2, 4, 3`.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Separar el último nodo para cada posición recorre la lista cada vez, O(n²): unos 5·10⁹ pasos para 10⁵ pasajeros.
