Prueba el caso más pequeño en el que el montículo tiene que descartar algo: `prices = [1, 5], k = 1`. ¿Qué precio queda al final?
---
Cada vez que el montículo supera `k` elementos se saca un precio. ¿Qué precio quita `heappop`, y es ese el que quieres descartar?
---
`heapq` es un montículo de mínimos, así que al sacar se quita el precio más barato y el montículo acaba con los `k` más caros. Para descartar el más caro, usa un montículo de máximos: mete `-p` y devuelve `-sum(heap)`. En TypeScript, crea el montículo con `(a, b) => a > b` para que salga primero el valor más grande.
