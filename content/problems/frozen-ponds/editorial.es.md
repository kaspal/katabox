## Enfoque: union-find sobre las celdas

Da a la celda `(r, c)` el número `r × w + c`. Una estructura de **conjuntos disjuntos** guarda, para cada celda, un puntero a su padre; seguir los punteros lleva al representante de su grupo.

1. Empieza con cada celda en su propio grupo.
2. Para cada celda helada, únela con la celda de su derecha y con la de abajo, si también están heladas. (Las vecinas de la izquierda y de arriba se cubren cuando el recorrido llega a ellas).
3. Cuenta las celdas heladas por representante y ordena los recuentos de mayor a menor.

```
para cada (r, c) helada:
    si (r, c + 1) está helada: unir(id(r, c), id(r, c + 1))
    si (r + 1, c) está helada: unir(id(r, c), id(r + 1, c))
tamaños = celdas heladas por buscar(celda)
devolver tamaños de mayor a menor
```

`buscar` acorta el camino a la mitad mientras sube, y `unir` cuelga el grupo pequeño del grande, así que cada operación cuesta casi tiempo constante.

### Paso a paso

En el ejemplo 1 las uniones son (0,0)–(0,1), (0,0)–(1,0), (0,4)–(1,4), (2,2)–(3,2) y (3,1)–(3,2). Los grupos son {(0,0), (0,1), (1,0)}, {(2,2), (3,1), (3,2)} y {(0,4), (1,4)}: tamaños `[3, 3, 2]`.

Un relleno por inundación (BFS o DFS desde cada celda helada sin visitar) da la misma respuesta en el mismo tiempo; union-find además sirve cuando las celdas se hielan de una en una.

**Complejidad:** O(r · c · α(r · c)) de tiempo, prácticamente lineal, más ordenar los tamaños, y O(r · c) de espacio. Dejar que cada celda copie la etiqueta más pequeña de sus vecinas hasta que nada cambie es correcto, pero un estanque largo y retorcido necesita una ronda por cada celda de su recorrido: O((r · c)²).
