## Enfoque: union-find con una cuenta

Guarda `clubes = n`. Para cada fusión `[a, b]`, busca los representantes de los dos jugadores; si son distintos, une los dos clubes y resta uno a `clubes`. Añade `clubes` después de cada fusión.

Los representantes salen de una estructura **union-find** (conjuntos disjuntos):

```
buscar(x):                     # sube por los padres hasta la raíz, acortando el camino
    mientras padre[x] != x:
        padre[x] = padre[padre[x]]
        x = padre[x]
    devolver x
unir(a, b):
    ra, rb = buscar(a), buscar(b)
    si ra == rb: devolver Falso
    cuelga el árbol pequeño del grande
    devolver Cierto
```

Con compresión de caminos y unión por tamaño, cada operación cuesta un tiempo amortizado casi constante (la inversa de la función de Ackermann α).

### Paso a paso

Ejemplo 1: `[0, 1]` une dos clubes (quedan 4), `[3, 4]` también (3), `[1, 0]` encuentra la misma raíz (siguen 3), `[1, 4]` une {0, 1} y {3, 4} (2).

**Complejidad:** O((n + m) α(n)) de tiempo y O(n) de memoria. Reetiquetar un club entero en cada fusión cuesta O(n) por fusión, O(n · m) en total.
