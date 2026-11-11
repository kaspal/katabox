## Enfoque: búsqueda en anchura

Si la salida o la meta es una estantería, devuelve -1. Si no, haz un BFS desde la celda superior izquierda:

1. Mete la salida en una cola con distancia 0 y márcala como visitada.
2. Saca la siguiente celda. Si es la meta, devuelve su distancia.
3. Para cada una de sus cuatro vecinas que esté dentro de la cuadrícula, sea libre y no esté visitada, márcala y métela con distancia + 1.
4. Si la cola se vacía, la meta es inalcanzable: devuelve -1.

El BFS saca las celdas por orden de distancia, así que la primera vez que sale la meta su distancia es la más corta. Marcar las celdas al meterlas (y no al sacarlas) hace que cada celda entre en la cola como mucho una vez.

### Paso a paso

```
0 0 0
1 1 0
0 0 0
```

| distancia | celdas alcanzadas |
| --- | --- |
| 0 | (0,0) |
| 1 | (0,1) |
| 2 | (0,2) |
| 3 | (1,2) |
| 4 | (2,2): la meta |

**Complejidad:** O(rows·cols) en tiempo y espacio: cada celda entra en la cola como mucho una vez y tiene cuatro vecinas.
