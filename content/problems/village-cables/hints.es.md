Una red más barata nunca contiene un ciclo: quitar el cable más caro de un ciclo mantiene todo conectado y ahorra dinero. Así que la respuesta usa exactamente `n − 1` cables.
---
Recorre los cables del más barato al más caro. Tiende un cable solo si sus dos aldeas aún no están conectadas; si no, cerraría un ciclo.
---
Usa union-find para responder "¿están ya conectadas estas dos aldeas?" en casi O(1). Ordena los cables por coste (algoritmo de Kruskal). Si al final se tendieron menos de `n − 1` cables, devuelve `-1`.
