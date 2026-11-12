## Enfoque: Dijkstra sobre (casilla, cupón usado)

Sin el cupón, el cruce más barato es un camino mínimo donde entrar en una casilla cuesta su peaje, y todos los costes son positivos: el algoritmo de Dijkstra. El cupón es una elección única, así que se mete **en el estado**: `(r, c, usado)`, dos capas de la cuadrícula.

Desde `(r, c, usado)` hacia un vecino `(r', c')`:

- paga `tolls[r'][c']` y sigue en la misma capa;
- si `usado` es falso, también puede pagar `tolls[r'][c'] // 2` y pasar a la capa `usado`.

La salida también tiene dos estados: `(0, 0, falso)` cuesta `tolls[0][0]` y `(0, 0, cierto)` cuesta `tolls[0][0] // 2`. Ejecuta Dijkstra desde ambos con un montículo de mínimos de `(coste, estado)`; la respuesta es `mín(coste(destino, falso), coste(destino, cierto))`.

Cada ruta con un cupón como mucho es un camino en este grafo por capas con el mismo coste, y cada camino en él es una ruta válida, así que el camino mínimo es la respuesta. Todos los costes de las aristas son no negativos (la mitad de un peaje ≥ 1 es ≥ 0), así que Dijkstra se puede aplicar. Entrar dos veces en una casilla nunca ayuda, porque los peajes son positivos.

### Paso a paso

Ejemplo 3: en la capa sin usar, lo más barato hasta abajo a la derecha es 153 (un 100 y un 50). En la capa usada, la mitad del 100 en `(0, 1)` da 1 + 50 + 1 + 50 + 1 = 103, más barato que la mitad de un 50 (1 + 100 + 1 + 25 + 1 = 128). Respuesta 103.

**Complejidad:** O(V log V) con V = 2 · filas · columnas estados y 4 movimientos cada uno (hasta 2 · 22 500 estados). Relajar cada casilla una y otra vez hasta que nada cambie (al estilo de Bellman–Ford) cuesta O(V²) en una ruta con muchas vueltas.
