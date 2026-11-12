## Enfoque: algoritmo de Kruskal con union-find

Buscamos un **árbol de expansión mínima**: el conjunto más barato de cables que conecta todas las aldeas. El algoritmo de Kruskal lo construye de forma voraz:

1. Ordena los cables por coste.
2. Recórrelos en orden. Si el cable une dos aldeas que aún no están conectadas, tiéndelo y fusiona sus grupos; si ya están conectadas, sáltalo (solo cerraría un ciclo).
3. Para tras `n − 1` cables. Si la lista se acaba antes, alguna aldea es inalcanzable: devuelve `-1`.

Por qué la elección voraz es segura: toma el cable más barato `e` entre el grupo que contiene la aldea `a` y el resto. Cualquier red que conecte todo tiene algún cable que cruza entre esos dos lados; cambiarlo por `e` mantiene la red conectada y no cuesta más. Así que alguna red más barata contiene `e`.

Una estructura **union-find** responde "¿mismo grupo?" y fusiona grupos en tiempo casi constante.

```
ordenar cables por coste
total = 0; tendidos = 0
para a, b, coste en cables:
    si unir(a, b):            # cierto cuando a y b estaban en grupos distintos
        total += coste; tendidos += 1
devolver total si tendidos == n - 1, si no -1
```

### Paso a paso

Ordenados: 1–2 (3), 2–3 (4), 0–2 (5), 0–1 (7), 1–3 (9). Se tiende 1–2, se tiende 2–3, se tiende 0–2: tres cables para cuatro aldeas, total 12. 0–1 y 1–3 cerrarían ciclos.

**Complejidad:** O(m log m) de tiempo por la ordenación más casi O(m) por las uniones, y O(n + m) de espacio. Hacer crecer la red de aldea en aldea recorriendo todos los cables en busca del más barato que sale de ella (un algoritmo de Prim sencillo) cuesta O(n · m).
