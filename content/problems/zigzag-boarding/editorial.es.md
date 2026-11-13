## Enfoque: partir, invertir la mitad trasera e intercalar

1. **Centro:** avanza `slow` un paso y `fast` dos pasos hasta que `fast` llegue al final. Con n nodos, la mitad delantera son los primeros ⌈n/2⌉ nodos.
2. **Cortar e invertir:** corta tras la mitad delantera e invierte la trasera sin memoria extra (tres punteros).
3. **Intercalar:** alterna un nodo de la mitad delantera y otro de la trasera invertida, reenlazándolos sobre la marcha.

### Paso a paso

`[3, 8, 1, 6, 9]`: delantera `[3, 8, 1]`, trasera `[6, 9]`, invertida `[9, 6]`. Al intercalar queda `3, 9, 8, 6, 1`.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Separar el último nodo para cada posición recorre la lista cada vez, O(n²): unos 5·10⁹ pasos para 10⁵ pasajeros.
