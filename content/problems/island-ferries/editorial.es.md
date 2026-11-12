## Enfoque: union-find

Guarda, para cada isla, un puntero `parent[i]`; la isla donde acaba una cadena de punteros es el **representante** de su grupo.

- `find(x)`: sigue `parent` hasta llegar a un representante. Hacer que cada isla visitada apunte a su abuela por el camino (reducción a la mitad) mantiene las cadenas cortas.
- `union(a, b)`: busca los dos representantes; si son distintos, cuelga el grupo pequeño del grande (unión por tamaño) y cuenta una fusión.

Al principio cada isla es su propio representante. Tras procesar todas las líneas, el número de grupos es `n − fusiones`.

### Paso a paso

`n = 6`, `links = [[0, 3], [4, 5], [3, 1]]`:

| línea | representantes | ¿fusión? | grupos |
| --- | --- | --- | --- |
| | | | 6 |
| [0, 3] | 0, 3 | sí | 5 |
| [4, 5] | 4, 5 | sí | 4 |
| [3, 1] | 0, 1 | sí | 3 |

**Complejidad:** O((n + m) α(n)) en tiempo, donde α es la inversa de la función de Ackermann (como mucho 4 en la práctica), y O(n) en espacio. Un recorrido en anchura sobre listas de adyacencia también es O(n + m). Reetiquetar las islas con la menor etiqueta de sus vecinas hasta que nada cambie puede necesitar una pasada por cada isla de una cadena larga: O(n·m).
