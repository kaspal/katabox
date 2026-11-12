## Enfoque: un montículo de mínimos con las k mejores

Recorre las puntuaciones y guarda las `k` mejores vistas hasta ahora en un **montículo de mínimos**, cuya cima es la menor guardada:

- mientras el montículo tenga menos de `k`, mete la nueva;
- si no, y la nueva supera a la cima, sustituye la cima por ella.

Al final el montículo tiene exactamente las `k` puntuaciones más altas (con repeticiones). Ordénalas de mayor a menor.

```
montículo = []
para s en scores:
    si tamaño < k: meter(montículo, s)
    si no, si s > cima: sustituir_cima(montículo, s)
devolver el montículo ordenado de mayor a menor
```

### Paso a paso

`scores = [132, 201, 87, 201, 156, 99]`, `k = 3`: el montículo pasa por {132}, {132, 201}, {87, 132, 201}. Después 201 sustituye a 87, 156 sustituye a 132 y 99 se queda corto. Ordenado: `[201, 201, 156]`.

**Complejidad:** O(n log k) de tiempo y O(k) de espacio adicional. Ordenar todas las puntuaciones cuesta O(n log n) y aquí también sirve; el montículo muestra el patrón que escala cuando `k` es pequeño y las puntuaciones no dejan de llegar.
