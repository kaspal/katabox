# Learn glossary (English ↔ Spanish)

The terms the Learn topics (ADR 0029) use, so theory, quizzes, narration and UI strings stay consistent in both languages. Add a row when a new topic introduces a term. Code identifiers (`heap`, `stack`, `low[u]`) stay in English inside code spans in both languages.

## General

| English | Spanish | Notes |
| --- | --- | --- |
| algorithm | algoritmo | |
| array | arreglo | "lista" when the sample uses a Python list as such |
| Big-O / cost | O grande / coste | "coste", never "costo" |
| best / average / worst case | caso mejor / medio / peor | |
| key | clave | never "llave" |
| loop | bucle | |
| pointer | puntero | never "apuntador" |
| recursion, call stack | recursión, pila de llamadas | |
| sorting, to sort | ordenación, ordenar | never "ordenamiento" |
| stable (sort) | estable | |
| step (of the animation) | paso | |
| string | cadena | |
| topic, subject | tema, materia | UI: "Materias" |
| learning path | ruta de aprendizaje | |
| See it animated | Míralo animado | Editorial tab link |

## Data structures

| English | Spanish | Notes |
| --- | --- | --- |
| stack | pila | |
| queue, ring buffer | cola, búfer circular | |
| deque | cola de dos extremos | |
| linked list | lista enlazada | its "next" is an *enlace* |
| hash table, bucket | tabla hash, cubeta | |
| separate chaining / open addressing | encadenamiento / direccionamiento abierto | |
| Bloom filter | filtro de Bloom | |
| heap, priority queue | montículo, cola de prioridad | |
| binary search tree | árbol binario de búsqueda | |
| trie | trie (árbol de prefijos) | |
| union-find | unión-búsqueda | |
| segment tree, Fenwick tree | árbol de segmentos, árbol de Fenwick | |

## Graphs and grids

| English | Spanish | Notes |
| --- | --- | --- |
| node / vertex | nodo | "vértice" only in *vértice de corte* |
| edge | arista | "enlace" only for real-world links (roads, pipes) and *enlace bajo* |
| directed / undirected | dirigido / no dirigido | |
| cycle | ciclo | "bucle" also appears in prose; prefer "ciclo" in new text |
| shortest path | camino más corto | "ruta más corta" on grids is fine |
| spanning tree | árbol de expansión | |
| strongly connected component | componente fuertemente conexa | the `scc` topic uses Kosaraju |
| bridge, articulation point | puente, punto de articulación (vértice de corte) | |
| low-link | enlace bajo | |
| max flow, residual capacity | flujo máximo, capacidad residual | |
| grid | cuadrícula | |

## Techniques

| English | Spanish | Notes |
| --- | --- | --- |
| dynamic programming | programación dinámica | |
| memoization | memoización | never "memorización" |
| greedy | voraz | |
| backtracking | vuelta atrás | "backtracking" may follow in parentheses |
| two pointers, sliding window | dos punteros, ventana deslizante | |
| prefix sums | sumas prefijas | |
| pivot | pivote | |
| convex hull | envolvente convexa | |
