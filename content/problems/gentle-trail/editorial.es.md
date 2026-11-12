## Enfoque: Dijkstra sobre el paso más duro

Define el `esfuerzo(ruta)` como su paso más duro. Queremos la ruta hasta el punto de abajo a la derecha con el menor esfuerzo. El algoritmo de Dijkstra funciona con cualquier "longitud" que no disminuya al alargar la ruta, y `max` cumple: alargar una ruta con un paso de dificultad `s` da `max(esfuerzo, s) ≥ esfuerzo`.

```
mejor[inicio] = 0; montículo = [(0, inicio)]
mientras montículo:
    d, punto = sacar el menor
    si punto es el final: devolver d
    si d > mejor[punto]: continuar       # una entrada antigua del montículo
    para cada vecino v:
        nd = max(d, |altura[v] - altura[punto]|)
        si nd < mejor[v]: mejor[v] = nd; meter (nd, v)
```

Cuando un punto sale del montículo, ninguna ruta puede llegar a él con menos esfuerzo, porque todas las demás entradas del montículo son al menos igual de grandes y los pasos nunca bajan el esfuerzo.

### Paso a paso

En el ejemplo 1 el montículo expande 10 (0), después 12 (2), 15 (3) y 17 (3). Desde 17 se ofrece el final, 40, con 23, y 50 con 33. Desde 10 se había ofrecido 30 con 20, así que sale 30 (20), después 31 (20) y 33 (20). Desde 33 se vuelve a ofrecer el final con `max(20, 7) = 20`, que mejora 23, y el final sale del montículo con 20.

### Otras formas

- **Búsqueda binaria sobre el límite** más un BFS por intento: O(r·c·log(altura máxima)).
- **Union-find:** ordena todas las parejas de vecinos por diferencia y únelas en orden hasta que inicio y final queden conectados; la última diferencia usada es la respuesta.

**Complejidad:** O(r·c·log(r·c)) de tiempo y O(r·c) de espacio. Probar cada dificultad de paso distinta en orden creciente, con una búsqueda nueva cada vez, cuesta O(r·c) por intento y hasta unos 2·r·c intentos.
