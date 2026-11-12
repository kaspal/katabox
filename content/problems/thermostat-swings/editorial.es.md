## Enfoque: dos colas monótonas

Para el **máximo**, guarda una cola doble de índices cuyas lecturas decrecen estrictamente de delante hacia atrás:

```
para i, r en enumerar(readings):
    mientras colamax y readings[colamax[-1]] <= r: colamax.pop()   # ya nunca serán el máximo
    colamax.añadir(i)
    si colamax[0] <= i - k: colamax.popleft()                      # salió de la ventana
```

Cuando la ventana termina en `i` (es decir, `i ≥ k − 1`), `readings[colamax[0]]` es su máximo. Cada índice que se quita por detrás está dominado: una lectura posterior al menos igual de grande se queda en la ventana al menos lo mismo. Así que la de delante siempre es la mayor lectura que sigue en la ventana.

Aplica la misma lógica con `>=` para el **mínimo** en una segunda cola, y devuelve `máx − mín` para cada ventana.

### Paso a paso

`[3, 1, 4, 1, 5, 9, 2, 6]`, k = 3. Para la primera ventana, la cola de máximos queda [4] y la de mínimos [1, 4] (valores): oscilación 3. En el índice 5 (valor 9) la cola de máximos pasa a [9] y la de mínimos a [1, 5, 9], así que la ventana [1, 5, 9] oscila 8.

**Complejidad:** O(n) de tiempo, porque cada índice entra y sale de cada cola una vez, y O(k) de memoria. Recorrer cada ventana cuesta O(n · k).
