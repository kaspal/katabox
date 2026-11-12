## Enfoque: una pila de días que siguen esperando

Recorre los días de izquierda a derecha con una pila de los índices que aún no encontraron un día más cálido. Sus temperaturas nunca suben desde el fondo de la pila hasta la cima: un día más cálido que la cima ya la habría sacado.

Para el día `i`:
1. Mientras la cima `j` cumpla `temps[j] < temps[i]`, sácala y pon `wait[j] = i − j`.
2. Apila `i`.

Los días que quedan en la pila al final se quedan con `wait = 0`.

Cada índice entra una vez y sale como mucho una vez, así que el trabajo total es lineal.

### Paso a paso

`temps = [3, 1, 4, 1, 5, 2]`:

| día | temp. | sacados (espera) | pila después |
| --- | --- | --- | --- |
| 0 | 3 | | [0] |
| 1 | 1 | | [0, 1] |
| 2 | 4 | 1 (1), 0 (2) | [2] |
| 3 | 1 | | [2, 3] |
| 4 | 5 | 3 (1), 2 (2) | [4] |
| 5 | 2 | | [4, 5] |

**Complejidad:** O(n) en tiempo y O(n) de espacio para la pila. Recorrer hacia delante desde cada día cuesta O(n²) cuando el tiempo se mantiene igual o frío durante mucho tiempo.
